//! # バックエンドAPIデータベース Lambda ハンドラー
//!
//! このモジュールは、AWS Lambda上で動作するコンタクトフォームAPIのエントリーポイントです。
//! Amazon API Gateway HTTP API からのリクエストを受け取り、JWTトークンによる認証を行った後、
//! HTTPメソッドに基づいて適切なハンドラーにルーティングします。
//!
//! ## アーキテクチャ概要
//!
//! ```text
//! クライアント
//!   └─▶ Amazon API Gateway HTTP API (JWT Authorizer)
//!         └─▶ AWS Lambda (このモジュール)
//!               └─▶ Amazon Aurora DSQL (SeaORM経由)
//! ```
//!
//! ## 認証フロー
//!
//! 1. クライアントは Amazon Cognito からJWT IDトークンを取得する
//! 2. JWT IDトークンを `Authorization: Bearer <token>` ヘッダーに付与してリクエストを送信する
//! 3. API Gateway の JWT Authorizer がトークンを検証する
//! 4. 検証済みのJWTクレーム（`email`、`sub`）が `requestContext.authorizer.jwt.claims` に格納される
//! 5. このモジュールがクレームを読み取り、ユーザーを識別する
//!
//! ## サポートするエンドポイント
//!
//! | メソッド | パス | 説明 |
//! |--------|------|------|
//! | GET | /inquiries | 認証済みユーザーのお問い合わせ一覧を取得 |
//! | POST | /inquiries | 新規お問い合わせを作成 |
//!
//! ## 環境変数
//!
//! | 変数名 | 必須 | 説明 |
//! |--------|------|------|
//! | `DSQL_ENDPOINT` | ✓ | Aurora DSQLクラスターのエンドポイント |
//! | `DSQL_REGION` | ✓ | Aurora DSQLクラスターのAWSリージョン |
//! | `CORS_ORIGIN` | - | 許可するCORSオリジン（デフォルト: `https://ngicf-testpage.pages.dev`） |
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use std::{env, sync::LazyLock};

mod db;
mod handlers;
mod models;

use db::create_db;
use handlers::{handle_get_inquiries, handle_post_inquiry};
use models::{Request, Response};

static CORS_ORIGIN: LazyLock<String> = LazyLock::new(|| {
    env::var("CORS_ORIGIN").unwrap_or_else(|_| "https://ngicf-testpage.pages.dev".to_string())
});

/// メインのLambda関数ハンドラー
///
/// API Gatewayからの受信HTTPリクエストを処理し、JWTで認証してから
/// HTTPメソッドに基づいて適切なハンドラーにルーティングします。
///
/// # 処理フロー
///
/// 1. リクエストコンテキストからJWTクレームを抽出する
/// 2. `email` クレームと `sub` クレームの存在・妥当性を検証する
/// 3. `DSQL_ENDPOINT` / `DSQL_REGION` 環境変数からデータベース接続を確立する
/// 4. HTTPメソッドに応じて [`handle_get_inquiries`] または [`handle_post_inquiry`] にディスパッチする
/// 5. 処理完了後、データベース接続を閉じる
/// 6. ハンドラーでエラーが発生した場合は HTTP 500 を返す
///
/// # Arguments
///
/// * `event` - API Gatewayリクエストを含むLambdaイベント。
///   [`Request`] 構造体にデシリアライズされ、リクエストコンテキスト（JWTクレームを含む）と
///   オプションのリクエストボディを持ちます。
///
/// # Returns
///
/// * `Ok(Response)` - API Gatewayに返すHTTPレスポンス。
///   正常時はハンドラーが返すレスポンスをそのまま返します。
///   内部エラー時は HTTP 500 レスポンスを返します。
/// * `Err(Error)` - Lambda ランタイムレベルの致命的なエラー（通常は発生しない）
///
/// # エラーレスポンス
///
/// | ステータスコード | エラー | 発生条件 |
/// |--------------|--------|---------|
/// | 401 | Unauthorized | JWTクレームに `email` または `sub` が存在しない、もしくは `sub` がUUID形式でない |
/// | 405 | Method Not Allowed | GET/POST以外のHTTPメソッドが使用された |
/// | 500 | INTERNAL_SERVER_ERROR | データベース接続エラー、クエリエラーなどの内部エラー |
///
/// # Authentication
///
/// すべてのリクエストには、Amazon CognitoからのJWT IDトークンが必要です。
/// トークンにはユーザーを識別するために使用される`email`クレームと`sub`クレームが含まれている必要があります。
/// `sub` クレームはCognito ユーザーの一意識別子（UUID v4形式）です。
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let (event, _context) = event.into_parts();

    let cors_origin = &*CORS_ORIGIN;

    let dsql_endpoint = env::var("DSQL_ENDPOINT").map_err(|_| {
        tracing::error!("DSQL_ENDPOINT environment variable is not set");
        anyhow::anyhow!("DSQL_ENDPOINT environment variable is not set")
    })?;

    let dsql_region = env::var("DSQL_REGION").map_err(|_| {
        tracing::error!("DSQL_REGION environment variable is not set");
        anyhow::anyhow!("DSQL_REGION environment variable is not set")
    })?;

    // JWTクレームからメールアドレスを抽出する
    let auth_info = event.request_context.authorizer.as_ref().and_then(|auth| {
        let email = auth.jwt.claims.email.as_deref()?;
        if email.is_empty() {
            return None;
        }
        let cognito_sub = auth.jwt.claims.cognito_sub.as_deref()?;
        if cognito_sub.is_empty() {
            return None;
        }
        match uuid::Uuid::parse_str(cognito_sub) {
            Ok(cognito_sub) => Some((email, cognito_sub)),
            Err(err) => {
                tracing::warn!("Invalid cognito_sub in JWT claims: {}", err);
                None
            }
        }
    });

    let (email, cognito_sub) = match auth_info {
        Some(auth_info) => auth_info,
        None => {
            return Ok(Response::error(
                401,
                "Unauthorized",
                "Invalid or missing required JWT claims (email and sub)",
                &cors_origin,
            ));
        }
    };

    // SeaORMデータベース接続を作成する
    let db = create_db("crudrole", &dsql_endpoint, &dsql_region).await?;

    let result = match event.request_context.http.method.as_str() {
        "GET" => handle_get_inquiries(&db, email, cognito_sub, &cors_origin).await,
        "POST" => {
            let body = event.body.as_deref().unwrap_or("");
            handle_post_inquiry(&db, email, cognito_sub, body, &cors_origin).await
        }
        _ => Ok(Response::error(
            405,
            "Method Not Allowed",
            "Method not allowed",
            &cors_origin,
        )),
    };

    // データベース接続を閉じる
    if let Err(err) = db.close().await {
        tracing::error!("Failed to close database connection: {:?}", err);
    }

    result.or_else(|e| {
        tracing::error!("Error processing request: {:?}", e);
        Ok(Response::error(
            500,
            "INTERNAL_SERVER_ERROR",
            "An error occurred while processing your request",
            &cors_origin,
        ))
    })
}

/// Lambda関数のエントリーポイント
///
/// ロギングを初期化してLambdaランタイムを起動します。
///
/// ## 初期化処理
///
/// 1. `tracing_subscriber` を INFO レベルで初期化します。ログにはターゲット名と時刻は含めません。
/// 2. Lambda ランタイムを起動し、[`function_handler`] をサービス関数として登録します。
/// 3. ランタイムはAWS Lambda環境からイベントを受け取り、[`function_handler`] を呼び出します。
///
/// # Returns
///
/// * `Ok(())` - ランタイムが正常に終了した場合（通常は発生しない）
/// * `Err(Error)` - ランタイムの初期化または実行中に致命的なエラーが発生した場合
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
