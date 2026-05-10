use crate::models::{
    CreateInquiryRequest, CreateInquiryResponse, Inquiry, InquiryListResponse, Response,
};
use lambda_runtime::Error;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbBackend, FromQueryResult, Set, Statement};
use sea_orm_entities::entity::inquiries;

pub(crate) async fn handle_get_inquiries(
    db: &DatabaseConnection,
    email: &str,
    cors_origin: &str,
) -> Result<Response, Error> {
    tracing::info!("Querying inquiries for email: {}", email);

    let inquiries: Vec<Inquiry> = Inquiry::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        "SELECT id, email, subject, body, created_at FROM get_inquiries_by_email($1) ORDER BY created_at DESC",
        [email.to_owned().into()],
    ))
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

pub(crate) async fn handle_post_inquiry(
    db: &DatabaseConnection,
    email: &str,
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
        email: Set(email.to_string()),
        subject: Set(create_request.subject.clone()),
        body: Set(create_request.body.clone()),
        created_at: Set(now),
    };

    new_inquiry.insert(db).await.map_err(|e| {
        tracing::error!("Failed to insert inquiry: {}", e);
        anyhow::anyhow!("Failed to insert inquiry: {}", e)
    })?;

    let inquiry = Inquiry {
        id,
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
