//! # データモデルモジュール
//!
//! このモジュールは、コンタクトフォームAPIのリクエスト・レスポンスに使用するデータ構造を定義します。
//! Lambda 関数が受け取る API Gateway リクエストの構造と、クライアントに返すレスポンスの構造を
//! 型安全に扱えるようにします。
//!
//! ## リクエスト構造
//!
//! API Gateway HTTP API が Lambda に渡すペイロードは次の形式です（バージョン2.0）：
//!
//! ```json
//! {
//!   "requestContext": {
//!     "http": { "method": "GET" },
//!     "authorizer": {
//!       "jwt": {
//!         "claims": {
//!           "email": "user@example.com",
//!           "sub": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
//!         }
//!       }
//!     }
//!   },
//!   "body": "{\"subject\":\"...\",\"body\":\"...\"}"
//! }
//! ```
//!
//! ## レスポンス構造
//!
//! Lambda から API Gateway に返すレスポンスは次の形式です：
//!
//! ```json
//! {
//!   "statusCode": 200,
//!   "headers": {
//!     "Content-Type": "application/json",
//!     "Access-Control-Allow-Origin": "https://example.com"
//!   },
//!   "body": "{\"email\":\"...\",\"count\":1,\"inquiries\":[...]}"
//! }
//! ```
//!
//! ## OpenAPI スキーマ
//!
//! `openapi` フィーチャーが有効な場合、[`utoipa::ToSchema`] が derive されます。
//! [`crate::bin::generate-openapi`] バイナリがこれを使用して OpenAPI 定義を生成します。

use sea_orm::FromQueryResult;
use sea_orm_entities::entity::inquiries;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// API Gateway HTTP API からの Lambda イベントペイロード
///
/// API Gateway HTTP API ペイロードフォーマットバージョン 2.0 に対応しています。
/// `serde(rename = "requestContext")` によって JSON の `requestContext` フィールドと対応します。
#[derive(Debug, Deserialize)]
pub(crate) struct Request {
    /// リクエストコンテキスト。HTTPメソッドや認可情報を含む。
    #[serde(rename = "requestContext")]
    pub(crate) request_context: RequestContext,
    /// リクエストボディ（JSON文字列）。POST リクエストの場合に設定される。
    /// API Gateway は Base64エンコードなしの場合は文字列として渡す。
    pub(crate) body: Option<String>,
}

/// API Gateway リクエストコンテキスト
///
/// `requestContext` フィールドに対応し、HTTPメソッドと認可情報を保持します。
#[derive(Debug, Deserialize)]
pub(crate) struct RequestContext {
    /// HTTPメソッド情報（`GET`、`POST` など）
    pub(crate) http: Http,
    /// JWT Authorizer による認可情報。JWT Authorizer が設定されている場合に存在する。
    /// Lambda 関数が直接呼び出された場合や認証なしの場合は `None`。
    pub(crate) authorizer: Option<Authorizer>,
}

/// HTTP メソッド情報
#[derive(Debug, Deserialize)]
pub(crate) struct Http {
    /// HTTPメソッド文字列（例: `"GET"`, `"POST"`, `"PUT"`, `"DELETE"`）
    pub(crate) method: String,
}

/// API Gateway JWT Authorizer の認可情報
///
/// `requestContext.authorizer` フィールドに対応します。
/// JWT Authorizer が検証済みのトークンクレームをここに設定します。
#[derive(Debug, Deserialize)]
pub(crate) struct Authorizer {
    /// JWT トークンの情報
    pub(crate) jwt: Jwt,
}

/// JWT トークン情報
///
/// JWT Authorizer が検証したトークンのクレーム情報を保持します。
#[derive(Debug, Deserialize)]
pub(crate) struct Jwt {
    /// JWT クレームセット
    pub(crate) claims: Claims,
}

/// JWT クレームセット
///
/// Cognito JWT IDトークンに含まれるクレームのうち、本 API が使用するものを定義します。
/// `sub` クレームは `cognito_sub` にリネームされます（`serde(rename = "sub")`）。
#[derive(Debug, Deserialize)]
pub(crate) struct Claims {
    /// 認証済みユーザーのメールアドレス。
    /// Cognito ユーザープールで `email` 属性が必須の場合に存在します。
    pub(crate) email: Option<String>,
    /// Cognito ユーザーの一意識別子（UUID v4 形式）。
    /// JWT 標準の `sub` クレームに対応します（`serde(rename = "sub")` により `sub` から読み取る）。
    #[serde(rename = "sub")]
    pub(crate) cognito_sub: Option<String>,
}

/// Lambda から API Gateway に返すHTTPレスポンス
///
/// API Gateway Lambda プロキシ統合のレスポンス形式に従います。
/// `statusCode`、`headers`、`body` フィールドが必要です。
/// `serde(rename = "statusCode")` により JSON の `statusCode` にシリアライズされます。
#[derive(Debug, Serialize)]
pub(crate) struct Response {
    /// HTTP ステータスコード（例: 200, 201, 401, 500）
    #[serde(rename = "statusCode")]
    pub(crate) status_code: u16,
    /// レスポンスヘッダー。常に `Content-Type: application/json` と
    /// `Access-Control-Allow-Origin: <cors_origin>` を含む。
    pub(crate) headers: HashMap<String, String>,
    /// レスポンスボディ（JSON文字列）。シリアライズ済みの JSON 文字列を格納する。
    pub(crate) body: String,
}

/// お問い合わせの詳細情報
///
/// データベースの `inquiries` テーブルの1レコードに対応します。
/// GET /inquiries レスポンスの `inquiries` 配列要素として使用されます。
/// `sea_orm::FromQueryResult` により SeaORM のクエリ結果から直接マッピングできます。
#[derive(Debug, Serialize, FromQueryResult)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub(crate) struct Inquiry {
    /// お問い合わせの一意識別子（UUID v7）。タイムスタンプを含むため作成順でソート可能。
    pub(crate) id: uuid::Uuid,
    /// Cognito ユーザーの一意識別子（UUID v4）。お問い合わせのオーナーを識別する。
    pub(crate) cognito_sub: uuid::Uuid,
    /// お問い合わせ送信者のメールアドレス。
    pub(crate) email: String,
    /// お問い合わせの件名。
    pub(crate) subject: String,
    /// お問い合わせの本文。
    pub(crate) body: String,
    /// お問い合わせ作成日時（タイムゾーンオフセット付き）。UTC で保存される。
    pub(crate) created_at: chrono::DateTime<chrono::FixedOffset>,
}

/// GET /inquiries のレスポンスボディ
///
/// 認証済みユーザーのお問い合わせ一覧を返します。
/// `inquiries` は作成日時の降順（新しい順）で格納されます。
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub(crate) struct InquiryListResponse {
    /// 認証済みユーザーのメールアドレス。リクエストの JWT クレームから取得。
    pub(crate) email: String,
    /// お問い合わせの件数。`inquiries` 配列の長さと等しい。
    pub(crate) count: u64,
    /// お問い合わせの一覧。作成日時の降順（新しい順）で格納される。
    pub(crate) inquiries: Vec<Inquiry>,
}

/// POST /inquiries のリクエストボディ
///
/// 新規お問い合わせの作成に必要なフィールドを定義します。
/// リクエストボディは JSON 形式で、`subject` と `body` フィールドが必須です。
///
/// # 使用例
///
/// ```json
/// {
///   "subject": "お問い合わせの件名",
///   "body": "お問い合わせの詳細内容"
/// }
/// ```
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub(crate) struct CreateInquiryRequest {
    /// お問い合わせの件名。空文字列も許容されるが、実際の運用では1文字以上が推奨される。
    pub(crate) subject: String,
    /// お問い合わせの本文。空文字列も許容されるが、実際の運用では1文字以上が推奨される。
    pub(crate) body: String,
}

/// POST /inquiries のレスポンスボディ
///
/// 作成されたお問い合わせの詳細情報を返します。
/// HTTP 201 Created と共に返されます。
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub(crate) struct CreateInquiryResponse {
    /// 新規作成されたお問い合わせの詳細情報。
    pub(crate) inquiry: Inquiry,
}

/// エラーレスポンスボディ
///
/// エラー発生時（4xx, 5xx）のレスポンスボディ形式を定義します。
/// `openapi` フィーチャー有効時のみ OpenAPI スキーマとして使用されます。
/// 実際のエラーレスポンスは [`Response::error`] メソッドで生成されます。
///
/// # 使用例
///
/// ```json
/// {
///   "error": "Unauthorized",
///   "message": "Invalid or missing required JWT claims"
/// }
/// ```
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[allow(dead_code)]
pub(crate) struct ErrorResponseBody {
    /// エラーの種類を示す短い識別子（例: `"Unauthorized"`, `"INTERNAL_SERVER_ERROR"`）
    pub(crate) error: String,
    /// エラーの詳細メッセージ。デバッグや表示に使用する。
    pub(crate) message: String,
}

/// [`inquiries::Model`] から [`Inquiry`] への変換
///
/// SeaORM のクエリ結果（`inquiries::Model`）を API レスポンス用の [`Inquiry`] 構造体に変換します。
/// 全フィールドを直接マッピングします。
impl From<inquiries::Model> for Inquiry {
    fn from(model: inquiries::Model) -> Self {
        Inquiry {
            id: model.id,
            cognito_sub: model.cognito_sub,
            email: model.email,
            subject: model.subject,
            body: model.body,
            created_at: model.created_at,
        }
    }
}

impl Response {
    /// 正常レスポンスを生成する
    ///
    /// 指定されたステータスコード、ボディ、CORSオリジンから [`Response`] を構築します。
    /// `Content-Type: application/json` と `Access-Control-Allow-Origin: <cors_origin>` ヘッダーを
    /// 自動的に設定します。
    ///
    /// # Arguments
    ///
    /// * `status_code` - HTTP ステータスコード（例: 200, 201）
    /// * `body` - レスポンスボディの値。[`serde_json::Value`] から文字列に変換されて格納される。
    /// * `cors_origin` - `Access-Control-Allow-Origin` ヘッダーに設定するオリジン文字列
    ///
    /// # Returns
    ///
    /// 構築された [`Response`] インスタンス
    pub(crate) fn new(status_code: u16, body: Value, cors_origin: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert(
            "Access-Control-Allow-Origin".to_string(),
            cors_origin.to_string(),
        );

        Response {
            status_code,
            headers,
            body: body.to_string(),
        }
    }

    /// エラーレスポンスを生成する
    ///
    /// [`Self::new`] のラッパーで、エラーレスポンス用の JSON ボディ
    /// `{"error": "<error>", "message": "<message>"}` を自動的に構築します。
    ///
    /// # Arguments
    ///
    /// * `status_code` - HTTP エラーステータスコード（例: 401, 405, 500）
    /// * `error` - エラーの種類を示す短い識別子（例: `"Unauthorized"`, `"INTERNAL_SERVER_ERROR"`）
    /// * `message` - エラーの詳細メッセージ
    /// * `cors_origin` - `Access-Control-Allow-Origin` ヘッダーに設定するオリジン文字列
    ///
    /// # Returns
    ///
    /// `{"error": "<error>", "message": "<message>"}` をボディとする [`Response`] インスタンス
    pub(crate) fn error(status_code: u16, error: &str, message: &str, cors_origin: &str) -> Self {
        Self::new(
            status_code,
            json!({
                "error": error,
                "message": message
            }),
            cors_origin,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_new_status_code() {
        let resp = Response::new(200, serde_json::json!({"ok": true}), "https://example.com");
        assert_eq!(resp.status_code, 200);
    }

    #[test]
    fn test_response_new_cors_header() {
        let origin = "https://example.com";
        let resp = Response::new(200, serde_json::json!({}), origin);
        assert_eq!(
            resp.headers
                .get("Access-Control-Allow-Origin")
                .map(String::as_str),
            Some(origin)
        );
    }

    #[test]
    fn test_response_new_content_type() {
        let resp = Response::new(200, serde_json::json!({}), "https://example.com");
        assert_eq!(
            resp.headers.get("Content-Type").map(String::as_str),
            Some("application/json")
        );
    }

    #[test]
    fn test_response_error_status_and_body() {
        let resp = Response::error(401, "Unauthorized", "Invalid token", "https://example.com");
        assert_eq!(resp.status_code, 401);
        let body: serde_json::Value = serde_json::from_str(&resp.body).unwrap();
        assert_eq!(body["error"], "Unauthorized");
        assert_eq!(body["message"], "Invalid token");
    }

    #[test]
    fn test_inquiry_serialization() {
        let id = uuid::Uuid::now_v7();
        let cognito_sub = uuid::Uuid::now_v7();
        let now = chrono::Utc::now().fixed_offset();
        let inquiry = Inquiry {
            id,
            cognito_sub,
            email: "test@example.com".to_string(),
            subject: "Test subject".to_string(),
            body: "Test body".to_string(),
            created_at: now,
        };
        let json = serde_json::to_value(&inquiry).unwrap();
        assert_eq!(json["email"], "test@example.com");
        assert_eq!(json["subject"], "Test subject");
        assert_eq!(json["body"], "Test body");
    }

    #[test]
    fn test_create_inquiry_request_deserialization() {
        let json = r#"{"subject": "Hello", "body": "World"}"#;
        let req: CreateInquiryRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.subject, "Hello");
        assert_eq!(req.body, "World");
    }

    #[test]
    fn test_claims_deserialization_with_missing_sub() {
        let json = r#"{"email":"test@example.com"}"#;
        let claims: Claims = serde_json::from_str(json).unwrap();
        assert_eq!(claims.email.as_deref(), Some("test@example.com"));
        assert_eq!(claims.cognito_sub, None);
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Response::new はどんなステータスコードとオリジンでも必ず生成できる
        #[test]
        fn prop_response_new_status_code_is_preserved(
            status_code in 100u16..=599u16,
            origin in "https?://[a-z]{1,10}\\.[a-z]{2,4}",
        ) {
            let resp = Response::new(status_code, serde_json::json!({}), &origin);
            prop_assert_eq!(resp.status_code, status_code);
        }

        /// Response::new は常に Access-Control-Allow-Origin ヘッダーを設定する
        #[test]
        fn prop_response_new_cors_header_matches_origin(
            origin in "https?://[a-z]{1,10}\\.[a-z]{2,4}",
        ) {
            let resp = Response::new(200, serde_json::json!({}), &origin);
            prop_assert_eq!(
                resp.headers.get("Access-Control-Allow-Origin").map(String::as_str),
                Some(origin.as_str())
            );
        }

        /// Response::new は常に Content-Type: application/json を設定する
        #[test]
        fn prop_response_new_content_type_is_always_json(
            status_code in 100u16..=599u16,
            origin in "https?://[a-z]{1,10}\\.[a-z]{2,4}",
        ) {
            let resp = Response::new(status_code, serde_json::json!({}), &origin);
            prop_assert_eq!(
                resp.headers.get("Content-Type").map(String::as_str),
                Some("application/json")
            );
        }

        /// Response::error はエラーと message フィールドを JSON body に含む
        #[test]
        fn prop_response_error_body_contains_error_and_message(
            status_code in 400u16..=599u16,
            error in "[A-Za-z_]{1,30}",
            message in "[A-Za-z0-9 ]{1,100}",
            origin in "https?://[a-z]{1,10}\\.[a-z]{2,4}",
        ) {
            let resp = Response::error(status_code, &error, &message, &origin);
            let body: serde_json::Value = serde_json::from_str(&resp.body).unwrap();
            prop_assert_eq!(body["error"].as_str(), Some(error.as_str()));
            prop_assert_eq!(body["message"].as_str(), Some(message.as_str()));
        }

        /// CreateInquiryRequest は任意の subject/body 文字列を受け入れる
        #[test]
        fn prop_create_inquiry_request_roundtrip(
            subject in ".*",
            body in ".*",
        ) {
            let json = serde_json::json!({ "subject": subject, "body": body });
            let req: CreateInquiryRequest = serde_json::from_value(json).unwrap();
            prop_assert_eq!(&req.subject, &subject);
            prop_assert_eq!(&req.body, &body);
        }

        /// Inquiry は任意のメールアドレス・件名・本文を正しくシリアライズする
        #[test]
        fn prop_inquiry_serialization_preserves_fields(
            email in "[a-z]{1,10}@[a-z]{1,8}\\.[a-z]{2,4}",
            subject in "[A-Za-z0-9 ]{1,50}",
            body in "[A-Za-z0-9 ]{1,200}",
        ) {
            let id = uuid::Uuid::now_v7();
            let cognito_sub = uuid::Uuid::now_v7();
            let now = chrono::Utc::now().fixed_offset();
            let inquiry = Inquiry { id, cognito_sub, email: email.clone(), subject: subject.clone(), body: body.clone(), created_at: now };
            let json = serde_json::to_value(&inquiry).unwrap();
            prop_assert_eq!(json["email"].as_str(), Some(email.as_str()));
            prop_assert_eq!(json["subject"].as_str(), Some(subject.as_str()));
            prop_assert_eq!(json["body"].as_str(), Some(body.as_str()));
        }
    }
}
