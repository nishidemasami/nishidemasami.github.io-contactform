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

| ワークフロー | 主なトリガー | 監視パス |
| --- | --- | --- |
| API | `main` / `develop` への `push`・`pull_request`・`workflow_dispatch` | `.github/workflows/api_cicd.yaml`, `api/**` |
| Cognito | `main` / `develop` への `push`・`pull_request`・`workflow_dispatch` | `.github/workflows/cognito_cicd.yaml`, `infrastructure/auth/**` |
| DB | `main` / `develop` への `push`・`pull_request`・`workflow_dispatch` | `.github/workflows/db_migrate.yaml`, `infrastructure/liquibase_migrate/**` |
| ドキュメント | `develop` への `push`・`pull_request`・`workflow_dispatch` | `.github/workflows/document_cicd.yaml`, `testpage/**` |
| Wiki 更新 | `workflow_dispatch` | なし |

## API デプロイ

`api_cicd.yaml` は 3 ジョブ構成です。

1. `validate`: `sam validate --lint`、`cargo check`、`cargo test`
2. `deploy`: Rust Lambda を `sam build` / `sam deploy`
3. `export_openapi`: デプロイ済み API Gateway から `/api/openapi.yaml` をエクスポートし、差分があればコミット

API 側は [認証](auth.md) の Export を JWT Authorizer に、[データベース](db.md) の endpoint を Lambda 環境変数に利用します。

## Cognito デプロイ

`cognito_cicd.yaml` は 2 ジョブ構成です。

1. `validate`: `sam validate --lint`
2. `deploy`: pull request 以外で Cognito リソースを SAM デプロイ

`Stage=${{ github.ref_name }}` を渡すため、`develop` ブランチは develop 環境、`main` ブランチは main 環境に対応します。

## DB デプロイとマイグレーション

`db_migrate.yaml` は 4 ジョブ構成です。

1. `validate`: DB 用 SAM テンプレートを検証
2. `deploy`: DSQL クラスターをデプロイし endpoint を取得
3. `migrate`: Liquibase で `changelog.xml` を適用
4. `generate`: `sea-orm-cli` で `infrastructure/sea_orm/src/entity` を更新し、差分があればコミット

`generate` ジョブでは DSQL の管理者トークンを URL エンコードして `DATABASE_URL` を組み立てています。

## ドキュメント配信

`document_cicd.yaml` は `develop` ブランチ向けのドキュメント配信フローです。主な処理は次の通りです。

1. `testpage/` の依存関係をインストール
2. TypeDoc で `testpage` の API ドキュメントを生成
3. Storybook をビルド
4. `docs/README.md` に外部リンクを追記して Honkit をビルド
5. Cognito / API の CloudFormation Export を読んで Next.js を静的ビルド
6. `_output/` を Cloudflare Pages に配信

このワークフローは、**実行時に `docs/README.md` へ追記してから Honkit をビルドする** ため、コミット済み Wiki と配信時の表紙に差分が出る点に注意が必要です。

## Wiki 更新フロー

`update-wiki.yml` は手動実行専用で、`docs/AGENTS.md` を前提に Copilot CLI へ Wiki 更新を依頼します。変更があれば `docs/` をコミットし、新規ブランチから Pull Request を作成します。

## 依存関係

| 上流 | 下流 | 意味 |
| --- | --- | --- |
| Cognito デプロイ | API デプロイ / testpage ビルド | JWT Authorizer とフロントエンド設定値を提供 |
| DB デプロイ | API デプロイ | Lambda が使う DSQL endpoint と DB スキーマを提供 |
| API デプロイ | OpenAPI 出力 / testpage ビルド | API Gateway URL と API 定義の生成元 |
| ドキュメント配信 | 公開ドキュメント | Honkit / Storybook / TypeDoc / testpage をまとめて公開 |
| Wiki 更新 | ドキュメント保守 | `docs/` の内容を継続的に同期 |

## 関連ページ

- [README](README.md)
- [API](api.md)
- [認証](auth.md)
- [データベース](db.md)
