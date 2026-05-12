//! # バックエンドAPIデータベース Lambda ハンドラー
use lambda_runtime::{Error, LambdaEvent, run, service_fn};
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
/// # Arguments
/// * `event` - API Gatewayリクエストを含むLambdaイベント
///
/// # Returns
/// * `Ok(Response)` - API Gatewayに返すHTTPレスポンス
/// * `Err(Error)` - リクエスト処理に失敗した場合
///
/// # Authentication
/// すべてのリクエストには、Amazon CognitoからのJWT IDトークンが必要です。
/// トークンにはユーザーを識別するために使用される`email`クレームと`sub`クレームが含まれている必要があります。
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
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
