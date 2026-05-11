# LiquiBase Migrate

## ローカル開発の開始手順

### 1. PostgreSQL を Docker で起動する

```bash
docker run --name contactform-postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_DB=postgres \
  -p 5432:5432 \
  -d postgres:17
```

### 2. Liquibase を `context=local` で実行する

`local` context では Aurora DSQL 専用の `CREATE INDEX ASYNC` / `AWS IAM GRANT` は実行されず、通常の PostgreSQL で実行可能な変更のみが適用されます。

```bash
liquibase update \
  --changelog-file=infrastructure/liquibase_migrate/changelog.xml \
  --contexts=local \
  --url=jdbc:postgresql://localhost:5432/postgres \
  --username=postgres \
  --password=postgres
```

### 3. ローカル PostgreSQL 接続が必要な Rust テスト

`api/lambda/src/*.rs` でローカル PostgreSQL への接続を伴うテストは、次の属性を付けてください。

```rust
#[ignore = "ローカルでPostgreSQL環境が必要なため。"]
```
