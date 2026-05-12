# CI/CD

## 概要

GitHub Actions は `.github/workflows/` 配下で管理され、現在は **API / 認証 / DB / ドキュメント配信 / Wiki 更新** の 5 系統に分かれています。

| ワークフロー | 目的 | 主なソース |
| --- | --- | --- |
| `api_cicd.yaml` | API の検証、デプロイ、OpenAPI エクスポート | [`../.github/workflows/api_cicd.yaml`](../.github/workflows/api_cicd.yaml) |
| `cognito_cicd.yaml` | Cognito SAM テンプレートの検証とデプロイ | [`../.github/workflows/cognito_cicd.yaml`](../.github/workflows/cognito_cicd.yaml) |
| `db_migrate.yaml` | DSQL デプロイ、Liquibase 実行、SeaORM 生成 | [`../.github/workflows/db_migrate.yaml`](../.github/workflows/db_migrate.yaml) |
| `document_cicd.yaml` | Honkit / Storybook / TypeDoc / testpage の生成と Cloudflare Pages 配信 | [`../.github/workflows/document_cicd.yaml`](../.github/workflows/document_cicd.yaml) |
| `update-wiki.yml` | Copilot CLI による `docs/` 更新と PR 作成 | [`../.github/workflows/update-wiki.yml`](../.github/workflows/update-wiki.yml) |

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

`api_cicd.yaml` は 3 ジョブ構成です。

1. `validate`: `sam validate --lint`、ローカル PostgreSQL 起動、Liquibase `--contexts=local`、`cargo check`、`cargo test -- --include-ignored`
2. `deploy`: pull request 以外で `sam build` と `sam deploy` を実行
3. `export_openapi`: デプロイ済み API Gateway から `api/openapi-${{ github.ref_name }}.yaml` をエクスポートし、差分があれば PR を作成

API 側は [認証](auth.md) の Export を JWT Authorizer に、[データベース](db.md) の endpoint を Lambda 環境変数に使います。

## Cognito デプロイ

`cognito_cicd.yaml` は 2 ジョブ構成です。

1. `validate`: `sam validate --lint`
2. `deploy`: pull request 以外で Cognito リソースを `sam deploy`

`Stage=${{ github.ref_name }}` を渡すため、push では `develop` / `release` を自動反映し、`main` は主に手動実行時にデプロイされます。

## DB デプロイとマイグレーション

`db_migrate.yaml` は 4 ジョブ構成です。

1. `validate`: DB 用 SAM テンプレートを検証
2. `deploy`: DSQL クラスターをデプロイし endpoint を取得
3. `migrate`: Liquibase で `changelog.xml` を `github.ref_name` コンテキスト付きで適用
4. `generate`: `sea-orm-cli generate entity` で `infrastructure/sea_orm/src/entity` を更新し、差分があれば PR を作成

`generate` ジョブでは Aurora DSQL の admin 認証トークンを URL エンコードして `DATABASE_URL` を組み立てています。

## ドキュメント配信

`document_cicd.yaml` は `develop` ブランチ向けのドキュメント配信フローです。主な処理は次の通りです。

1. API 側で `cargo doc --no-deps` を実行
2. `testpage/` で `npm ci`、`npm run lint`、TypeDoc 生成、Storybook ビルド、Next.js ビルドを実行
3. 作業ツリー上の `docs/README.md` に外部リンクを追記して Honkit をビルド
4. Cognito / API の CloudFormation Export を読んで `testpage` を本番用設定で静的ビルド
5. OpenAPI をエクスポートし、Swagger UI を `_output/raw/openapi/` に生成
6. `_output/` を Cloudflare Pages に配信

このワークフローは、**ビルド時の作業ツリーで `docs/README.md` にリンクを追記してから Honkit を生成する** ため、コミット済みファイルと公開ページの表紙に一時的な差分が生じます。

## Wiki 更新フロー

`update-wiki.yml` は `main` への push または手動実行で動き、`docs/AGENTS.md` を前提に Copilot CLI へ Wiki 更新を依頼します。変更があれば `docs/` をコミットし、新規ブランチを作成して `main` 向け Pull Request を作成します。

## 依存関係

| 上流 | 下流 | 意味 |
| --- | --- | --- |
| Cognito デプロイ | API デプロイ / testpage ビルド | JWT Authorizer とフロントエンド設定値を提供 |
| DB デプロイ | API デプロイ | Lambda が使う DSQL endpoint と DB スキーマを提供 |
| API デプロイ | OpenAPI 出力 / testpage ビルド | API Gateway URL と API 定義の生成元 |
| ドキュメント配信 | 公開ドキュメント | Honkit / Storybook / TypeDoc / testpage / Swagger UI をまとめて公開 |
| Wiki 更新 | ドキュメント保守 | `docs/` の内容を継続的に同期 |

## 関連ページ

- [README](README.md)
- [FAQ](FAQ.md)
- [API](api.md)
- [認証](auth.md)
- [データベース](db.md)
- [更新ログ](log.md)
