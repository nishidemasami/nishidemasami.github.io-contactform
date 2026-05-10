# CI/CD

## 概要

GitHub Actions は `.github/workflows/` 配下で管理され、認証基盤、データベース基盤、Wiki 更新の 3 系統に分かれています。

| ワークフロー | 目的 | 主なソース |
| --- | --- | --- |
| `cognito_cicd.yaml` | Cognito SAM テンプレートの検証とデプロイ | [`../.github/workflows/cognito_cicd.yaml`](../.github/workflows/cognito_cicd.yaml) |
| `db_migrate.yaml` | DSQL デプロイ、Liquibase 実行、SeaORM 生成 | [`../.github/workflows/db_migrate.yaml`](../.github/workflows/db_migrate.yaml) |
| `update-wiki.yml` | Copilot CLI による Wiki 更新と PR 作成 | [`../.github/workflows/update-wiki.yml`](../.github/workflows/update-wiki.yml) |

## ブランチと実行契機

`cognito_cicd.yaml` と `db_migrate.yaml` は次の条件で起動します。

- `main` / `develop` への `push`
- `main` / `develop` 向け `pull_request`
- 手動実行 (`workflow_dispatch`)

どちらも変更パスで対象ディレクトリを絞っています。

| ワークフロー | 監視パス |
| --- | --- |
| Cognito | `.github/workflows/cognito_cicd.yaml`, `infrastructure/auth/**` |
| DB | `.github/workflows/db_migrate.yaml`, `infrastructure/liquibase_migrate/**` |

## Cognito デプロイ

`cognito_cicd.yaml` は 2 ジョブ構成です。

1. `validate`: `aws-sam-cli` をインストールして `sam validate --lint` を実行
2. `deploy`: `pull_request` 以外で SAM デプロイを実行

`Stage=${{ github.ref_name }}` を渡すため、`develop` ブランチは develop 環境、`main` ブランチは main 環境に対応します。詳細は [認証](auth.md) を参照してください。

## DB デプロイとマイグレーション

`db_migrate.yaml` は 4 ジョブ構成です。

1. `validate`: DB 用 SAM テンプレートを検証
2. `deploy`: DSQL クラスターをデプロイして endpoint を取得
3. `migrate`: Liquibase で `changelog.xml` を適用
4. `generate`: SeaORM エンティティを生成し、差分があればコミット

`generate` ジョブでは AWS DSQL の管理者トークンを生成し、URL エンコードしたうえで `sea-orm-cli generate entity` を実行します。DB 構成の詳細は [データベース](db.md) を参照してください。

## Wiki 更新フロー

`update-wiki.yml` は手動実行専用です。処理内容は次の通りです。

1. Node.js 24 と GitHub Copilot CLI をセットアップ
2. `docs/AGENTS.md` の指示で Wiki 更新を実行
3. `docs/` 配下の変更をコミット
4. 新規ブランチを push して Pull Request を作成

このワークフローは `COPILOT_GITHUB_TOKEN` と `GITHUB_TOKEN` を使用します。Wiki の入口は [README](README.md)、目次は [SUMMARY](SUMMARY.md) です。

## 依存関係

| 上流 | 下流 | 意味 |
| --- | --- | --- |
| Cognito デプロイ | API 実装 | JWT 検証に必要な Issuer / Client 情報を提供 |
| DB デプロイ | API 実装 | 保存先テーブル、関数、ORM 定義を提供 |
| Wiki 更新 | ドキュメント運用 | インフラや運用変更を `docs/` に反映 |

## 関連ページ

- [API](api.md)
- [認証](auth.md)
- [データベース](db.md)
