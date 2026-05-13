use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{env, fs, path::PathBuf};
use utoipa::{
    Modify, OpenApi, ToSchema,
    openapi::{
        OpenApi as OpenApiDoc,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
        server::Server,
    },
};

#[derive(Serialize, Deserialize, ToSchema)]
struct Inquiry {
    id: uuid::Uuid,
    cognito_sub: uuid::Uuid,
    email: String,
    subject: String,
    body: String,
    created_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Serialize, Deserialize, ToSchema)]
struct InquiryListResponse {
    email: String,
    count: u64,
    inquiries: Vec<Inquiry>,
}

#[derive(Serialize, Deserialize, ToSchema)]
struct CreateInquiryRequest {
    subject: String,
    body: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
struct CreateInquiryResponse {
    inquiry: Inquiry,
}

#[derive(Serialize, Deserialize, ToSchema)]
struct ErrorResponse {
    error: String,
    message: String,
}

#[utoipa::path(
    get,
    path = "/inquiries",
    responses(
        (status = 200, description = "Get inquiries", body = InquiryListResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    security(
        ("CognitoAuthorizer" = [])
    )
)]
#[allow(dead_code)]
fn get_inquiries() {}

#[utoipa::path(
    post,
    path = "/inquiries",
    request_body = CreateInquiryRequest,
    responses(
        (status = 201, description = "Create inquiry", body = CreateInquiryResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    security(
        ("CognitoAuthorizer" = [])
    )
)]
#[allow(dead_code)]
fn post_inquiry() {}

struct ServerAddon {
    url: String,
}

impl Modify for ServerAddon {
    fn modify(&self, openapi: &mut OpenApiDoc) {
        openapi.servers = Some(vec![Server::new(self.url.clone())]);
    }
}

struct SecurityAddon;

impl Modify for SecurityAddon {
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

#[derive(OpenApi)]
#[openapi(
    paths(get_inquiries, post_inquiry),
    components(schemas(
        Inquiry,
        InquiryListResponse,
        CreateInquiryRequest,
        CreateInquiryResponse,
        ErrorResponse
    )),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

fn build_openapi(api_endpoint: String) -> OpenApiDoc {
    let mut openapi = ApiDoc::openapi();
    ServerAddon { url: api_endpoint }.modify(&mut openapi);
    openapi
}

fn output_path() -> PathBuf {
    if let Ok(path) = env::var("OPENAPI_OUTPUT") {
        return PathBuf::from(path);
    }

    if let Some(arg) = env::args().nth(1) {
        return PathBuf::from(arg);
    }

    PathBuf::from("../openapi.yaml")
}

fn api_endpoint() -> String {
    env::var("API_ENDPOINT")
        .unwrap_or_else(|_| "https://example.execute-api.ap-northeast-3.amazonaws.com".to_string())
}

fn cognito_issuer() -> Option<String> {
    let user_pool_id = env::var("USER_POOL_ID").ok()?;
    let region = env::var("AWS_REGION").unwrap_or_else(|_| "ap-northeast-3".to_string());
    Some(format!(
        "https://cognito-idp.{region}.amazonaws.com/{user_pool_id}"
    ))
}

fn cognito_client_id() -> Option<String> {
    env::var("CLIENT_ID").ok()
}

fn apply_cognito_security(
    openapi: OpenApiDoc,
    issuer: Option<String>,
    client_id: Option<String>,
) -> Value {
    let mut value = serde_json::to_value(openapi).expect("OpenAPI serialization should succeed");

    let mut security_scheme = json!({
        "type": "oauth2",
        "flows": {}
    });

    if let (Some(issuer), Some(client_id)) = (issuer, client_id) {
        security_scheme["x-amazon-apigateway-authorizer"] = json!({
            "identitySource": "$request.header.Authorization",
            "jwtConfiguration": {
                "audience": [client_id],
                "issuer": issuer
            },
            "type": "jwt"
        });
    }

    value["components"]["securitySchemes"]["CognitoAuthorizer"] = security_scheme;
    value
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = output_path();
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    let openapi = build_openapi(api_endpoint());
    let openapi = apply_cognito_security(openapi, cognito_issuer(), cognito_client_id());
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
    fn openapi_contains_expected_server_url() {
        let openapi = build_openapi("https://example.com".to_string());
        let servers = openapi.servers.expect("servers should be set");
        assert_eq!(servers[0].url, "https://example.com");
    }

    #[test]
    fn cognito_security_scheme_contains_authorizer_when_inputs_exist() {
        let openapi = build_openapi("https://example.com".to_string());
        let openapi = apply_cognito_security(
            openapi,
            Some("https://cognito-idp.ap-northeast-3.amazonaws.com/pool".to_string()),
            Some("client-id".to_string()),
        );
        assert_eq!(
            openapi["components"]["securitySchemes"]["CognitoAuthorizer"]["x-amazon-apigateway-authorizer"]
                ["jwtConfiguration"]["issuer"],
            "https://cognito-idp.ap-northeast-3.amazonaws.com/pool"
        );
        assert_eq!(
            openapi["components"]["securitySchemes"]["CognitoAuthorizer"]["x-amazon-apigateway-authorizer"]
                ["jwtConfiguration"]["audience"][0],
            "client-id"
        );
    }
}
