use crate::models::{
    CreateInquiryRequest, CreateInquiryResponse, Inquiry, InquiryListResponse, Response,
};
#[cfg(feature = "openapi")]
use crate::models::ErrorResponseBody;
use lambda_runtime::Error;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use sea_orm_entities::entity::inquiries::{self, Column, Entity as Inquiries};

#[cfg_attr(
    feature = "openapi",
    utoipa::path(
        get,
        path = "/inquiries",
        tag = "inquiries",
        responses(
            (status = 200, description = "Get inquiries", body = InquiryListResponse),
            (status = 401, description = "Unauthorized", body = ErrorResponseBody),
            (status = 500, description = "Internal server error", body = ErrorResponseBody),
        ),
        security(
            ("CognitoAuthorizer" = []),
            ("BearerAuth" = []),
        )
    )
)]
pub(crate) async fn handle_get_inquiries(
    db: &DatabaseConnection,
    email: &str,
    cognito_sub: uuid::Uuid,
    cors_origin: &str,
) -> Result<Response, Error> {
    tracing::info!("Querying inquiries for email: {}", email);

    let inquiries: Vec<Inquiry> = Inquiries::find()
        .filter(Column::Email.eq(email))
        .filter(Column::CognitoSub.eq(cognito_sub))
        .order_by_desc(Column::CreatedAt)
        .into_model::<Inquiry>()
        .all(db)
        .await
        .map_err(|e| {
            tracing::error!("Database query failed: {}", e);
            anyhow::anyhow!("Database query failed: {}", e)
        })?;

    let response_body = InquiryListResponse {
        email: email.to_string(),
        count: inquiries.len() as u64,
        inquiries,
    };

    Ok(Response::new(
        200,
        serde_json::to_value(response_body)?,
        cors_origin,
    ))
}

#[cfg_attr(
    feature = "openapi",
    utoipa::path(
        post,
        path = "/inquiries",
        tag = "inquiries",
        request_body = CreateInquiryRequest,
        responses(
            (status = 201, description = "Create inquiry", body = CreateInquiryResponse),
            (status = 401, description = "Unauthorized", body = ErrorResponseBody),
            (status = 500, description = "Internal server error", body = ErrorResponseBody),
        ),
        security(
            ("CognitoAuthorizer" = []),
            ("BearerAuth" = []),
        )
    )
)]
pub(crate) async fn handle_post_inquiry(
    db: &DatabaseConnection,
    email: &str,
    cognito_sub: uuid::Uuid,
    body: &str,
    cors_origin: &str,
) -> Result<Response, Error> {
    tracing::info!("Creating inquiry for email: {}", email);

    let create_request: CreateInquiryRequest = serde_json::from_str(body).map_err(|e| {
        tracing::error!("Failed to parse request body: {}", e);
        anyhow::anyhow!("Invalid request body: {}", e)
    })?;

    let id = uuid::Uuid::now_v7();
    let now = chrono::Utc::now().fixed_offset();

    let new_inquiry = inquiries::ActiveModel {
        id: Set(id),
        cognito_sub: Set(cognito_sub),
        email: Set(email.to_string()),
        subject: Set(create_request.subject.clone()),
        body: Set(create_request.body.clone()),
        created_at: Set(now),
        reply: Set(None),
        respondent: Set(None),
        reply_at: Set(None),
    };

    new_inquiry.insert(db).await.map_err(|e| {
        tracing::error!("Failed to insert inquiry: {}", e);
        anyhow::anyhow!("Failed to insert inquiry: {}", e)
    })?;

    let inquiry = Inquiry {
        id,
        cognito_sub,
        email: email.to_string(),
        subject: create_request.subject,
        body: create_request.body,
        created_at: now,
    };

    let response_body = CreateInquiryResponse { inquiry };

    Ok(Response::new(
        201,
        serde_json::to_value(response_body)?,
        cors_origin,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use sea_orm::{ActiveModelTrait, ColumnTrait, Database, EntityTrait, QueryFilter, Set};
    use sea_orm_entities::entity::inquiries::{Column, Entity as Inquiries};

    const ORDERING_OFFSET_SECS: i64 = 1;

    async fn connect_local_test_db() -> DatabaseConnection {
        let database_url = std::env::var("LOCAL_TEST_DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@localhost:5432/postgres?sslmode=disable".to_string()
        });
        Database::connect(database_url)
            .await
            .expect("Failed to connect local PostgreSQL test DB")
    }

    async fn cleanup_test_inquiries(db: &DatabaseConnection, email: &str) {
        Inquiries::delete_many()
            .filter(Column::Email.eq(email))
            .exec(db)
            .await
            .expect("test inquiry cleanup should succeed");
    }

    #[tokio::test]
    #[ignore = "ローカルのDBが必要なためデフォルトでは実行しない"]
    async fn test_handle_post_inquiry_with_local_postgres() {
        let db = connect_local_test_db().await;
        let email = format!("local-post-{}@example.com", uuid::Uuid::now_v7());
        let cognito_sub = uuid::Uuid::now_v7();
        let body = r#"{"subject":"subject from test","body":"body from test"}"#;
        cleanup_test_inquiries(&db, &email).await;

        let db_for_test = db.clone();
        let email_for_test = email.clone();
        let test_result = tokio::spawn(async move {
            let response =
                handle_post_inquiry(&db_for_test, &email_for_test, cognito_sub, body, "https://example.com")
                    .await
                    .expect("handle_post_inquiry should succeed");

            assert_eq!(response.status_code, 201);
            let response_body: serde_json::Value =
                serde_json::from_str(&response.body).expect("response body should be valid JSON");
            assert_eq!(response_body["inquiry"]["email"], email_for_test);
            assert_eq!(response_body["inquiry"]["subject"], "subject from test");
            assert_eq!(response_body["inquiry"]["body"], "body from test");

            let inquiry_id = uuid::Uuid::parse_str(
                response_body["inquiry"]["id"]
                    .as_str()
                    .expect("response should include inquiry id"),
            )
            .expect("inquiry id should be valid UUID");
            let saved = Inquiries::find_by_id(inquiry_id)
                .one(&db_for_test)
                .await
                .expect("DB query should succeed")
                .expect("inserted inquiry should exist");
            assert_eq!(saved.email, email_for_test);
            assert_eq!(saved.cognito_sub, cognito_sub);
        })
        .await;

        cleanup_test_inquiries(&db, &email).await;

        if let Err(err) = test_result {
            if err.is_panic() {
                std::panic::resume_unwind(err.into_panic());
            }
            panic!("test task failed: {err}");
        }
    }

    #[tokio::test]
    #[ignore = "ローカルのDBが必要なためデフォルトでは実行しない"]
    async fn test_handle_get_inquiries_with_local_postgres() {
        let db = connect_local_test_db().await;
        let cognito_sub = uuid::Uuid::now_v7();
        let email = format!("local-get-{}@example.com", uuid::Uuid::now_v7());
        let now = chrono::Utc::now().fixed_offset();
        cleanup_test_inquiries(&db, &email).await;

        inquiries::ActiveModel {
            id: Set(uuid::Uuid::now_v7()),
            cognito_sub: Set(cognito_sub),
            email: Set(email.clone()),
            subject: Set("older subject".to_string()),
            body: Set("older body".to_string()),
            reply: Set(None),
            respondent: Set(None),
            created_at: Set(now - Duration::seconds(ORDERING_OFFSET_SECS)),
            reply_at: Set(None),
        }
        .insert(&db)
        .await
        .expect("older test record insert should succeed");

        inquiries::ActiveModel {
            id: Set(uuid::Uuid::now_v7()),
            cognito_sub: Set(cognito_sub),
            email: Set(email.clone()),
            subject: Set("newer subject".to_string()),
            body: Set("newer body".to_string()),
            reply: Set(None),
            respondent: Set(None),
            created_at: Set(now),
            reply_at: Set(None),
        }
        .insert(&db)
        .await
        .expect("newer test record insert should succeed");

        let response = handle_get_inquiries(&db, &email, cognito_sub, "https://example.com")
            .await
            .expect("handle_get_inquiries should succeed");

        assert_eq!(response.status_code, 200);
        let response_body: serde_json::Value =
            serde_json::from_str(&response.body).expect("response body should be valid JSON");
        assert_eq!(response_body["email"], email);
        assert_eq!(response_body["count"], 2);
        let inquiries = response_body["inquiries"]
            .as_array()
            .expect("inquiries should be an array");
        assert_eq!(inquiries.len(), 2);
        assert_eq!(inquiries[0]["subject"], "newer subject");
        assert_eq!(inquiries[1]["subject"], "older subject");
        cleanup_test_inquiries(&db, &email).await;
    }
}
