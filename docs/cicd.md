# CI/CD

## 概要

GitHub Actions は `.github/workflows/` 配下で管理され、現在は **API / 認証 / DB / ドキュメント配信 / Wiki 更新** の 5 系統に分かれています。

| ワークフロー | 目的 | 主なソース |
| --- | --- | --- |
| `api_cicd.yaml` | API の検証、デプロイ、OpenAPI 生成物公開、カバレッジ公開 | [`../.github/workflows/api_cicd.yaml`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/.github/workflows/api_cicd.yaml) |
| `cognito_cicd.yaml` | Cognito SAM テンプレートの検証とデプロイ | [`../.github/workflows/cognito_cicd.yaml`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/.github/workflows/cognito_cicd.yaml) |
| `db_migrate.yaml` | DSQL デプロイ、Liquibase 実行、SeaORM 生成 | [`../.github/workflows/db_migrate.yaml`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/.github/workflows/db_migrate.yaml) |
| `document_cicd.yaml` | Honkit / Storybook / TypeDoc / testpage / OpenAPI ビューア生成と Cloudflare Pages 配信 | [`../.github/workflows/document_cicd.yaml`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/.github/workflows/document_cicd.yaml) |
| `update-wiki.yml` | Copilot CLI による `docs/` 更新と PR 作成 | [`../.github/workflows/update-wiki.yml`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/.github/workflows/update-wiki.yml) |

## 実行契機

| ワークフロー | 主なトリガー | 監視パス / 条件 |
| --- | --- | --- |
| API | `develop` / `release` への `push`、`main` / `develop` / `release` 向け `pull_request`、`workflow_dispatch` | `.github/workflows/api_cicd.yaml`, `api/**` |
| Cognito | `develop` / `release` への `push`、`main` / `develop` / `release` 向け `pull_request`、`workflow_dispatch` | `.github/workflows/cognito_cicd.yaml`, `infrastructure/auth/**` |
| DB | `develop` / `release` への `push`、`main` / `develop` / `release` 向け `pull_request`、`workflow_dispatch` | `.github/workflows/db_migrate.yaml`, `infrastructure/liquibase_migrate/**` |
| ドキュメント | `develop` への `push` / `pull_request`、`workflow_dispatch` | `.github/workflows/document_cicd.yaml`, `testpage/**` |
| Wiki 更新 | `main` への `push`、`workflow_dispatch` | パスフィルタなし |

`document_cicd.yaml` は `docs/**` を監視していないため、**Wiki だけを更新しても自動では再配信されません**。公開ドキュメントへ反映したい場合は `workflow_dispatch` などの別トリガーが必要です。

## API デプロイ

`api_cicd.yaml` は 4 ジョブ構成です。

1. `validate`: `sam validate --lint`、ローカル PostgreSQL 起動、Liquibase `--contexts=local`、OpenAPI 生成検証、`cargo check`、`cargo test -- --include-ignored`、`cargo tarpaulin`
2. `deploy`: pull request 以外で `sam build` と `sam deploy` を実行
3. `export_openapi`: Cognito / API の CloudFormation Export を取得して `cargo run --features openapi --bin generate-openapi` を実行し、`api/openapi-${branch}.yaml` を GitHub Release と Deployment として公開
4. `publish_coverage`: Tarpaulin HTML レポートを GitHub Release と Deployment として公開

API 側は [認証](auth.md) の Export を JWT Authorizer に、[データベース](db.md) の endpoint を Lambda 環境変数に使います。

## Cognito デプロイ

`cognito_cicd.yaml` は 2 ジョブ構成です。

1. `validate`: `sam validate --lint`
2. `deploy`: pull request 以外で Cognito リソースを `sam deploy`

`Stage=${{ github.ref_name }}` を渡すため、push では `develop` / `release` を自動反映し、`main` は主に手動実行でデプロイされます。

## DB デプロイとマイグレーション

`db_migrate.yaml` は 4 ジョブ構成です。

1. `validate`: DB 用 SAM テンプレートを検証
2. `deploy`: DSQL クラスターをデプロイし endpoint を取得
3. `migrate`: Liquibase で `changelog.xml` を `github.ref_name` コンテキスト付きで適用
4. `generate`: `sea-orm-cli generate entity` で `infrastructure/sea_orm/src/entity` を更新し、差分があれば対象ブランチ向け PR を作成

`generate` ジョブでは Aurora DSQL の admin 認証トークンを URL エンコードして `DATABASE_URL` を組み立てています。

## ドキュメント配信

`document_cicd.yaml` は 2 ジョブ構成です。

1. `validate`: API 用 `sam validate --lint`、OpenAPI 生成検証、`cargo doc --no-deps`、`testpage` の lint / 型チェック / Storybook / Next.js ビルド、PlantUML SVG 生成を実行
2. `generate-api-docs`: Rustdoc / TypeDoc / Storybook / Honkit / OpenAPI ビューア / `testpage` をまとめて `_output/` に組み立て、Cloudflare Pages へ配信

`generate-api-docs` ジョブの中では、次の生成物が公開用 `_output/` に集約されます。

- `docs/` を Honkit でビルドした Wiki
- `api/lambda` の Rustdoc
- `testpage` の TypeDoc
- Storybook
- `api/openapi-${branch}.yaml` と RapiDoc / OpenAPI Explorer / Swagger Editor / Scalar / Redoc / Stoplight / SwaggerUI
- `testpage/out` の静的 Next.js 出力

このワークフローは、**Honkit ビルド前に `docs/README.md` の末尾へ公開リンクを追記してから生成**するため、コミット済みファイルと公開ページの表紙に一時的な差分が生じます。

## Wiki 更新フロー

`update-wiki.yml` は `main` への push または手動実行で動き、`docs/AGENTS.md` を前提に Copilot CLI へ Wiki 更新を依頼します。変更があれば `docs/` をコミットし、新規ブランチを作成して `main` 向け Pull Request を作成します。

このワークフローは `pnpm` と Node.js をセットアップしたうえで `@github/copilot` をグローバルインストールし、次のプロンプトで Wiki 更新を実行します。

```text
task Review and update the wiki files under docs/ based on the instructions in docs/AGENTS.md. Maintain consistency, update cross-references, and keep all pages current.
```

## 品質担保の仕組み

- **OpenAPI を Rust 実装から生成**: `api/lambda/src/bin/generate-openapi.rs` を `api_cicd.yaml` と `document_cicd.yaml` が実行し、API 契約と公開ドキュメントをコードから再生成します。
- **Liquibase で環境差異を抑制**: API 検証ではローカル PostgreSQL を起動し、`infrastructure/liquibase_migrate/changelog.xml` を `--contexts=local` で適用してから Rust テストを実行します。
- **Rust の検証を自動化**: API ワークフローは `cargo check`、`cargo test -- --include-ignored`、`cargo tarpaulin` を実行し、カバレッジ HTML も公開します。
- **フロントエンドの静的検証**: ドキュメント配信ワークフローは `testpage/` に対して ESLint、TypeScript 型チェック、Storybook、Next.js ビルドを行います。

## 依存関係

| 上流 | 下流 | 意味 |
| --- | --- | --- |
| Cognito デプロイ | API デプロイ / OpenAPI 生成 / testpage ビルド | JWT Authorizer とフロントエンド設定値を提供 |
| DB デプロイ | API デプロイ | Lambda が使う DSQL endpoint と DB スキーマを提供 |
| API デプロイ | OpenAPI 公開 / testpage ビルド | API Gateway URL と OpenAPI `servers` の生成元 |
| ドキュメント配信 | 公開ドキュメント | Honkit / Storybook / TypeDoc / testpage / OpenAPI ビューアをまとめて公開 |
| Wiki 更新 | ドキュメント保守 | `docs/` の内容を継続的に同期 |

## 関連ページ

- [README](README.md)
- [FAQ](FAQ.md)
- [API](api.md)
- [認証](auth.md)
- [データベース](db.md)
- [更新ログ](log.md)
