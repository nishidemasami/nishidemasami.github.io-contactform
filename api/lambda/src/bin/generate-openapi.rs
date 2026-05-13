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
            components.add_security_scheme(
                "BearerAuth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some("JWTによるBearerトークン認証"))
                        .build(),
                ),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
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

fn get_required_env(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let value = env::var(name)?;
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed == "None" {
        return Err(format!("{name} must not be empty or None").into());
    }
    Ok(trimmed.to_string())
}

fn api_endpoint() -> Result<String, Box<dyn std::error::Error>> {
    get_required_env("API_ENDPOINT")
}

fn cognito_issuer() -> Result<String, Box<dyn std::error::Error>> {
    let user_pool_id = get_required_env("USER_POOL_ID")?;
    let region = env::var("AWS_REGION").unwrap_or_else(|_| "ap-northeast-3".to_string());
    Ok(format!(
        "https://cognito-idp.{region}.amazonaws.com/{user_pool_id}"
    ))
}

fn cognito_client_id() -> Result<String, Box<dyn std::error::Error>> {
    get_required_env("CLIENT_ID")
}

fn apply_cognito_security(openapi: OpenApiDoc, issuer: String, client_id: String) -> Value {
    let mut value = serde_json::to_value(openapi).expect("OpenAPI serialization should succeed");
    value["components"]["securitySchemes"]["CognitoAuthorizer"]["x-amazon-apigateway-authorizer"] =
        json!({
            "identitySource": "$request.header.Authorization",
            "jwtConfiguration": {
                "audience": [client_id],
                "issuer": issuer
            },
            "type": "jwt"
        });
    value
}

fn apply_metadata(mut openapi: Value) -> Value {
    openapi["openapi"] = json!("3.0.1");
    openapi["info"]["title"] = json!("Contact Form API");
    openapi["info"]["description"] = json!("API for contact form inquiries.");
    openapi["info"]["license"] = json!({ "name": "Proprietary" });
    openapi
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = output_path();
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    let openapi = build_openapi(api_endpoint()?);
    let openapi = apply_cognito_security(openapi, cognito_issuer()?, cognito_client_id()?);
    let openapi = apply_metadata(openapi);
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
        );
        assert_eq!(
            openapi["components"]["securitySchemes"]["CognitoAuthorizer"]
                ["x-amazon-apigateway-authorizer"]["jwtConfiguration"]["issuer"],
            "https://cognito-idp.ap-northeast-3.amazonaws.com/pool"
        );
    }
}
