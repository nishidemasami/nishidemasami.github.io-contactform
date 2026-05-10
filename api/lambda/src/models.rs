use sea_orm::FromQueryResult;
use sea_orm_entities::entity::inquiries;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub(crate) struct Request {
    #[serde(rename = "requestContext")]
    pub(crate) request_context: RequestContext,
    pub(crate) body: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RequestContext {
    pub(crate) http: Http,
    pub(crate) authorizer: Option<Authorizer>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Http {
    pub(crate) method: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Authorizer {
    pub(crate) jwt: Jwt,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Jwt {
    pub(crate) claims: Claims,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Claims {
    pub(crate) email: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct Response {
    #[serde(rename = "statusCode")]
    pub(crate) status_code: u16,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: String,
}

#[derive(Debug, Serialize, FromQueryResult)]
pub(crate) struct Inquiry {
    pub(crate) id: uuid::Uuid,
    pub(crate) cognito_sub: uuid::Uuid,
    pub(crate) email: String,
    pub(crate) subject: String,
    pub(crate) body: String,
    pub(crate) created_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Debug, Serialize)]
pub(crate) struct InquiryListResponse {
    pub(crate) email: String,
    pub(crate) count: u64,
    pub(crate) inquiries: Vec<Inquiry>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CreateInquiryRequest {
    pub(crate) subject: String,
    pub(crate) body: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct CreateInquiryResponse {
    pub(crate) inquiry: Inquiry,
}

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
            let now = chrono::Utc::now().fixed_offset();
            let inquiry = Inquiry { id, email: email.clone(), subject: subject.clone(), body: body.clone(), created_at: now };
            let json = serde_json::to_value(&inquiry).unwrap();
            prop_assert_eq!(json["email"].as_str(), Some(email.as_str()));
            prop_assert_eq!(json["subject"].as_str(), Some(subject.as_str()));
            prop_assert_eq!(json["body"].as_str(), Some(body.as_str()));
        }
    }
}
