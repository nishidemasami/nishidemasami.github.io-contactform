use aurora_dsql_sqlx_connector::pool;
use lambda_runtime::Error;
use sea_orm::{DatabaseConnection, SqlxPostgresConnector};

fn build_connection_string(role: &str, endpoint: &str, region: &str) -> String {
    format!("postgres://{role}@{endpoint}/postgres?region={region}")
}

pub(crate) async fn create_db(
    role: &str,
    endpoint: &str,
    region: &str,
) -> Result<DatabaseConnection, Error> {
    tracing::info!("Creating database connection with Aurora DSQL SQLx connector...");
    let connection_string = build_connection_string(role, endpoint, region);
    let pool = pool::connect(&connection_string)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

    Ok(SqlxPostgresConnector::from_sqlx_postgres_pool(pool))
}

#[cfg(test)]
mod tests {
    use super::build_connection_string;

    #[test]
    fn test_build_connection_string() {
        let endpoint = "foo0bar1baz2quux3quuux4.dsql.ap-northeast-1.on.aws";
        let region = "ap-northeast-1";
        let role = "selectview";

        let result = build_connection_string(role, endpoint, region);

        assert_eq!(
            result,
            "postgres://selectview@foo0bar1baz2quux3quuux4.dsql.ap-northeast-1.on.aws/postgres?region=ap-northeast-1"
        );
    }
}

#[cfg(test)]
mod prop_tests {
    use super::build_connection_string;
    use proptest::prelude::*;

    proptest! {
        /// 任意のエンドポイントとリージョンで接続文字列が正しい形式になる
        #[test]
        fn prop_connection_string_contains_endpoint_and_region(
            endpoint in "[a-z0-9][a-z0-9\\-]{0,30}\\.[a-z0-9\\.]{1,30}",
            region in "[a-z]{2,6}-[a-z]{4,10}-[0-9]",
        ) {
            let role = "selectview";
            let result = build_connection_string(role, &endpoint, &region);
            let role_prefix = format!("postgres://{role}@");
            prop_assert!(result.starts_with(&role_prefix));
            prop_assert!(result.contains(&endpoint));
            let region_param = format!("region={}", region);
            prop_assert!(result.contains(&region_param));
            prop_assert!(result.contains("/postgres?"));
        }

        /// 接続文字列は常に postgres:// スキームで始まる
        #[test]
        fn prop_connection_string_scheme(
            endpoint in "[a-z0-9\\.]{5,40}",
            region in "[a-z0-9\\-]{5,20}",
        ) {
            let role = "selectview";
            let result = build_connection_string(role, &endpoint, &region);
            let role_prefix = format!("postgres://{role}@");
            prop_assert!(result.starts_with(&role_prefix));
        }
    }
}
