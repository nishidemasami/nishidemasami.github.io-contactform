//! # OpenAPI 定義生成バイナリ
//!
//! このバイナリは、Rust コードから OpenAPI 定義（YAML形式）を生成します。
//! [`utoipa`] クレートで定義された API スキーマと [`SecurityAddon`]、[`ServerAddon`] を組み合わせ、
//! Amazon API Gateway 用の拡張フィールド (`x-amazon-apigateway-authorizer`) を付与した
//! OpenAPI 3.0.1 定義を出力します。
//!
//! ## 使用方法
//!
//! ```bash
//! cargo run --features openapi --bin generate-openapi
//! ```
//!
//! ## 環境変数
//!
//! | 変数名 | 必須 | 説明 |
//! |--------|------|------|
//! | `API_ENDPOINT` | ✓ | API エンドポイント URL（例: `https://abc123.execute-api.ap-northeast-3.amazonaws.com`） |
//! | `USER_POOL_ID` | ✓ | Cognito ユーザープール ID（例: `ap-northeast-3_AbCdEfGhI`）。Issuer URL の構築に使用。 |
//! | `CLIENT_ID` | ✓ | Cognito ユーザープールクライアント ID。JWT audience の設定に使用。 |
//! | `AWS_REGION` | - | AWS リージョン（デフォルト: `ap-northeast-3`）。Issuer URL の構築に使用。 |
//! | `OPENAPI_OUTPUT` | - | 出力ファイルパス（デフォルト: `../openapi.yaml`）。コマンドライン引数でも指定可能。 |
//!
//! ## 出力例
//!
//! 生成される YAML ファイルは OpenAPI 3.0.1 形式で、次のセクションを含みます：
//! - `info`: API 名、バージョン、ライセンス情報
//! - `servers`: API エンドポイント URL
//! - `paths`: 各エンドポイントの定義（GET/POST /inquiries）
//! - `components.schemas`: リクエスト・レスポンスのスキーマ定義
//! - `components.securitySchemes`: Cognito JWT Authorizer と Bearer 認証の定義
//!
//! ## CI/CD での使用
//!
//! `api_cicd.yaml` の `export_openapi` ジョブと `document_cicd.yaml` の `generate-api-docs` ジョブで
//! このバイナリが実行されます。実際の AWS CloudFormation エクスポートから取得した値を使用するため、
//! API デプロイ後に実行する必要があります。

use serde_json::{Value, json};
use std::{env, fs, path::PathBuf};
use utoipa::{
    Modify, OpenApi,
    openapi::{
        OpenApi as OpenApiDoc,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
        server::Server,
    },
};

#[path = "../models.rs"]
#[allow(dead_code)]
mod models;
#[path = "../handlers.rs"]
#[allow(dead_code)]
mod handlers;

/// OpenAPI の `servers` フィールドにエンドポイント URL を追加するモディファイア
///
/// [`utoipa::Modify`] を実装し、[`ApiDoc::openapi()`] で生成した OpenAPI ドキュメントに
/// API エンドポイント URL を追加します。
struct ServerAddon {
    /// 追加するサーバー URL（例: `https://abc123.execute-api.ap-northeast-3.amazonaws.com`）
    url: String,
}

impl Modify for ServerAddon {
    /// OpenAPI ドキュメントの `servers` フィールドを指定 URL で上書きする
    fn modify(&self, openapi: &mut OpenApiDoc) {
        openapi.servers = Some(vec![Server::new(self.url.clone())]);
    }
}

/// OpenAPI の `components.securitySchemes` に Cognito Authorizer を追加するモディファイア
///
/// [`utoipa::Modify`] を実装し、`CognitoAuthorizer` という名前の HTTP Bearer 認証スキームを
/// OpenAPI ドキュメントのセキュリティスキームに追加します。
/// 後処理（[`apply_cognito_security`]）で Amazon API Gateway 固有の
/// `x-amazon-apigateway-authorizer` 拡張フィールドが付与されます。
struct SecurityAddon;

impl Modify for SecurityAddon {
    /// `CognitoAuthorizer` セキュリティスキームを追加する
    ///
    /// `components.securitySchemes.CognitoAuthorizer` に HTTP Bearer JWT スキームを追加します。
    fn modify(&self, openapi: &mut OpenApiDoc) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "CognitoAuthorizer",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

/// OpenAPI ドキュメントのルート定義
///
/// [`utoipa::OpenApi`] derive マクロにより、`info`、`paths`、`components`、
/// `modifiers` が静的に定義されます。
/// 実行時にエンドポイント URL と Cognito 設定が動的に追加されます。
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Contact Form API",
        description = "API for contact form inquiries.",
        version = "0.1.0",
        license(name = "Proprietary")
    ),
    paths(handlers::handle_get_inquiries, handlers::handle_post_inquiry),
    components(schemas(
        models::Inquiry,
        models::InquiryListResponse,
        models::CreateInquiryRequest,
        models::CreateInquiryResponse,
        models::ErrorResponseBody
    )),
    modifiers(&SecurityAddon),
)]
struct ApiDoc;

/// OpenAPI ドキュメントを構築する
///
/// [`ApiDoc::openapi()`] でベースドキュメントを生成し、[`ServerAddon`] で
/// サーバー URL を追加します。セキュリティスキームは [`SecurityAddon`] により
/// `ApiDoc` の derive 時に追加済みです。
///
/// # Arguments
///
/// * `api_endpoint` - API エンドポイント URL（例: `https://abc123.execute-api.ap-northeast-3.amazonaws.com`）
///
/// # Returns
///
/// サーバー URL が設定された [`OpenApiDoc`] インスタンス
fn build_openapi(api_endpoint: String) -> OpenApiDoc {
    let mut openapi = ApiDoc::openapi();
    ServerAddon { url: api_endpoint }.modify(&mut openapi);
    openapi
}

/// OpenAPI 定義の出力パスを決定する
///
/// 以下の優先順で出力先を決定します：
/// 1. 環境変数 `OPENAPI_OUTPUT` が設定されている場合、その値をパスとして使用
/// 2. コマンドライン引数の第1引数が指定されている場合、その値をパスとして使用
/// 3. いずれも指定されていない場合、`../openapi.yaml` をデフォルトとして使用
///
/// # Returns
///
/// 出力ファイルの [`PathBuf`]
fn output_path() -> PathBuf {
    if let Ok(path) = env::var("OPENAPI_OUTPUT") {
        return PathBuf::from(path);
    }

    if let Some(arg) = env::args().nth(1) {
        return PathBuf::from(arg);
    }

    PathBuf::from("../openapi.yaml")
}

/// 必須環境変数を取得する
///
/// 指定した名前の環境変数を読み取り、空でないことと `"None"` でないことを検証します。
/// CI/CD で CloudFormation Export が存在しない場合に `"None"` という文字列が設定されることがあるため、
/// これを明示的に弾きます。
///
/// # Arguments
///
/// * `name` - 環境変数名
///
/// # Returns
///
/// * `Ok(String)` - トリム済みの環境変数値
/// * `Err` - 環境変数が設定されていない場合、空文字列の場合、または `"None"` の場合
///
/// # Errors
///
/// - 環境変数が未設定: [`std::env::VarError`] に由来するエラー
/// - 空文字列または `"None"`: `"<name> must not be empty or None"` メッセージのエラー
fn get_required_env(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let value = env::var(name)?;
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed == "None" {
        return Err(format!("{name} must not be empty or None").into());
    }
    Ok(trimmed.to_string())
}

/// API エンドポイント URL を取得する
///
/// 環境変数 `API_ENDPOINT` から API のベース URL を取得します。
/// この URL は OpenAPI ドキュメントの `servers[0].url` に設定されます。
///
/// # Returns
///
/// * `Ok(String)` - API エンドポイント URL（例: `https://abc123.execute-api.ap-northeast-3.amazonaws.com`）
/// * `Err` - 環境変数が設定されていない、空、または `"None"` の場合
fn api_endpoint() -> Result<String, Box<dyn std::error::Error>> {
    get_required_env("API_ENDPOINT")
}

/// Cognito JWT Issuer URL を構築する
///
/// 環境変数 `USER_POOL_ID` と `AWS_REGION`（デフォルト: `ap-northeast-3`）から
/// Cognito の Issuer URL を構築します。
/// この URL は OpenAPI の `x-amazon-apigateway-authorizer.jwtConfiguration.issuer` に設定されます。
///
/// # Returns
///
/// * `Ok(String)` - Cognito Issuer URL（例: `https://cognito-idp.ap-northeast-3.amazonaws.com/ap-northeast-3_AbCd`）
/// * `Err` - `USER_POOL_ID` 環境変数が設定されていない、空、または `"None"` の場合
fn cognito_issuer() -> Result<String, Box<dyn std::error::Error>> {
    let user_pool_id = get_required_env("USER_POOL_ID")?;
    let region = env::var("AWS_REGION").unwrap_or_else(|_| "ap-northeast-3".to_string());
    Ok(format!(
        "https://cognito-idp.{region}.amazonaws.com/{user_pool_id}"
    ))
}

/// Cognito ユーザープールクライアント ID を取得する
///
/// 環境変数 `CLIENT_ID` から Cognito ユーザープールクライアント ID を取得します。
/// この ID は OpenAPI の `x-amazon-apigateway-authorizer.jwtConfiguration.audience` に設定されます。
///
/// # Returns
///
/// * `Ok(String)` - Cognito ユーザープールクライアント ID
/// * `Err` - 環境変数が設定されていない、空、または `"None"` の場合
fn cognito_client_id() -> Result<String, Box<dyn std::error::Error>> {
    get_required_env("CLIENT_ID")
}

/// OpenAPI ドキュメントに Amazon API Gateway Cognito Authorizer 設定を付与する
///
/// [`utoipa`] が生成した OpenAPI ドキュメントを [`serde_json::Value`] に変換し、
/// `components.securitySchemes.CognitoAuthorizer` に Amazon API Gateway 固有の
/// `x-amazon-apigateway-authorizer` 拡張フィールドを追加します。
///
/// ## 追加されるフィールド
///
/// ```json
/// "x-amazon-apigateway-authorizer": {
///   "identitySource": "$request.header.Authorization",
///   "jwtConfiguration": {
///     "audience": ["<client_id>"],
///     "issuer": "<issuer>"
///   },
///   "type": "jwt"
/// }
/// ```
///
/// # Arguments
///
/// * `openapi` - [`build_openapi`] で生成した OpenAPI ドキュメント
/// * `issuer` - Cognito JWT Issuer URL（[`cognito_issuer`] で構築）
/// * `client_id` - Cognito ユーザープールクライアント ID（[`cognito_client_id`] で取得）
///
/// # Returns
///
/// * `Ok(Value)` - `x-amazon-apigateway-authorizer` が追加された JSON 値
/// * `Err` - OpenAPI ドキュメントの構造が予期しない形式の場合
///
/// # Errors
///
/// - `components` フィールドがオブジェクトでない場合: `"components must be an object"` エラー
/// - `securitySchemes` フィールドがオブジェクトでない場合: `"securitySchemes must be an object"` エラー
/// - `CognitoAuthorizer` フィールドがオブジェクトでない場合: `"CognitoAuthorizer must be an object"` エラー
fn apply_cognito_security(
    openapi: OpenApiDoc,
    issuer: String,
    client_id: String,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut value = serde_json::to_value(openapi).expect("OpenAPI serialization should succeed");

    let components = value
        .get_mut("components")
        .and_then(Value::as_object_mut)
        .ok_or("components must be an object")?;
    let security_schemes = components
        .get_mut("securitySchemes")
        .and_then(Value::as_object_mut)
        .ok_or("securitySchemes must be an object")?;
    let cognito_authorizer = security_schemes
        .get_mut("CognitoAuthorizer")
        .and_then(Value::as_object_mut)
        .ok_or("CognitoAuthorizer must be an object")?;

    cognito_authorizer.insert(
        "x-amazon-apigateway-authorizer".to_string(),
        json!({
            "identitySource": "$request.header.Authorization",
            "jwtConfiguration": {
                "audience": [client_id],
                "issuer": issuer
            },
            "type": "jwt"
        }),
    );
    Ok(value)
}

/// OpenAPI バージョンを 3.0.1 に設定する
///
/// [`utoipa`] が生成する OpenAPI バージョン文字列を `3.0.1` に上書きします。
/// Amazon API Gateway は OpenAPI 3.0.x を要求するため、この変換が必要です。
///
/// # Arguments
///
/// * `openapi` - バージョンフィールドを上書きする対象の JSON 値
///
/// # Returns
///
/// `openapi` フィールドが `"3.0.1"` に設定された JSON 値
fn apply_openapi_version(mut openapi: Value) -> Value {
    openapi["openapi"] = json!("3.0.1");
    openapi
}

/// OpenAPI 定義生成のエントリーポイント
///
/// 環境変数から設定を読み取り、OpenAPI 定義を YAML ファイルとして生成します。
///
/// ## 処理フロー
///
/// 1. [`output_path`] で出力先ファイルパスを決定する
/// 2. 出力先の親ディレクトリを作成する（存在しない場合）
/// 3. [`api_endpoint`] で API エンドポイント URL を取得する
/// 4. [`build_openapi`] で OpenAPI ドキュメントのベースを構築する
/// 5. [`apply_cognito_security`] で Cognito Authorizer 設定を追加する
/// 6. [`apply_openapi_version`] でバージョンを `3.0.1` に設定する
/// 7. YAML にシリアライズしてファイルに書き込む
/// 8. 出力パスを標準出力に表示する
///
/// # Returns
///
/// * `Ok(())` - 正常に OpenAPI YAML が生成された場合
/// * `Err` - 環境変数の取得、YAML シリアライズ、ファイル書き込みに失敗した場合
///
/// # Errors
///
/// - 必須環境変数が未設定・空・`"None"`: [`get_required_env`] のエラー
/// - ディレクトリ作成失敗: [`std::fs::create_dir_all`] のエラー
/// - YAML シリアライズ失敗: [`serde_yaml::to_string`] のエラー
/// - ファイル書き込み失敗: [`std::fs::write`] のエラー
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = output_path();
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    let openapi = build_openapi(api_endpoint()?);
    let openapi = apply_cognito_security(openapi, cognito_issuer()?, cognito_client_id()?)?;
    let openapi = apply_openapi_version(openapi);
    let yaml = serde_yaml::to_string(&openapi)?;
    fs::write(&output, yaml)?;

    println!("OpenAPI definition generated: {}", output.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openapi_contains_expected_path() {
        let openapi = build_openapi("https://example.com".to_string());
        assert!(openapi.paths.paths.contains_key("/inquiries"));
    }

    #[test]
    fn openapi_contains_bearer_auth_scheme() {
        let openapi = build_openapi("https://example.com".to_string());
        let value = serde_json::to_value(openapi).expect("should serialize openapi");
        assert_eq!(
            value["components"]["securitySchemes"]["BearerAuth"]["type"],
            "http"
        );
        assert_eq!(
            value["components"]["securitySchemes"]["BearerAuth"]["scheme"],
            "bearer"
        );
    }

    #[test]
    fn cognito_security_scheme_contains_authorizer() {
        let openapi = build_openapi("https://example.com".to_string());
        let openapi = apply_cognito_security(
            openapi,
            "https://cognito-idp.ap-northeast-3.amazonaws.com/pool".to_string(),
            "client-id".to_string(),
        )
        .expect("should apply cognito security");
        assert_eq!(
            openapi["components"]["securitySchemes"]["CognitoAuthorizer"]
                ["x-amazon-apigateway-authorizer"]["jwtConfiguration"]["issuer"],
            "https://cognito-idp.ap-northeast-3.amazonaws.com/pool"
        );
    }
}
