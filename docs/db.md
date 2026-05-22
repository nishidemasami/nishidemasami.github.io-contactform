# データベース

## 概要

データベース基盤は `infrastructure/liquibase_migrate/` で管理されています。AWS SAM で **Aurora DSQL クラスター** を作成し、Liquibase でスキーマを適用し、その結果をもとに GitHub Actions が `infrastructure/sea_orm/src/entity/` を再生成します。

- テンプレート: [`../infrastructure/liquibase_migrate/template.yaml`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/infrastructure/liquibase_migrate/template.yaml)
- 変更管理: [`../infrastructure/liquibase_migrate/changelog.xml`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/infrastructure/liquibase_migrate/changelog.xml)
- 補足 README: [`../infrastructure/liquibase_migrate/README.md`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/infrastructure/liquibase_migrate/README.md)
- SeaORM 生成物: [`../infrastructure/sea_orm/src/entity/`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/infrastructure/sea_orm/src/entity/)
- 自動化: [CI/CD](cicd.md)

## DSQL クラスター

| 項目 | 内容 |
| --- | --- |
| リソース | `AWS::DSQL::Cluster` |
| Stage | `develop`, `main`, `release` |
| 主な Outputs | `DSQLClusterIdentifier`, `DSQLClusterEndpoint` |
| Export 名 | `${StackNamePrefix}-db-${Stage}-DSQLClusterIdentifier`, `${StackNamePrefix}-db-${Stage}-DSQLClusterEndpoint` |

`db_migrate.yaml` は `snngicf-db-${branch}` スタックをデプロイし、CloudFormation 出力から endpoint を取得して Liquibase と SeaORM 生成に再利用します。[API](api.md) 側の `DSQL_ENDPOINT` Import 名もこの Export と一致しています。

## スキーマ

### テーブル

| テーブル | 主なカラム | 用途 |
| --- | --- | --- |
| `inquiries` | `id`, `cognito_sub`, `email`, `subject`, `body`, `reply`, `respondent`, `created_at`, `reply_at` | 問い合わせ保存と返信管理 |
| `users` | `id`, `cognito_sub`, `email`, `username`, `hashed_password`, `created_at` | ユーザー情報保存 |

現行の API / `testpage` が直接使っているのは `inquiries` テーブルです。`users` テーブルはスキーマには存在しますが、今回参照したソースには利用コードが見当たりません。

### インデックス

| インデックス | 対象 | 目的 |
| --- | --- | --- |
| `idx_inquiries_cognito_sub` | `inquiries(cognito_sub)` | ユーザー単位の検索高速化 |
| `idx_inquiries_created_at` | `inquiries(created_at)` | 新しい問い合わせ順の取得を補助 |

### 関数と権限

| 要素 | 内容 |
| --- | --- |
| 関数 | `get_inquiries_by_email(p_email VARCHAR(255))` |
| DB ロール | `selectview WITH LOGIN`, `crudrole WITH LOGIN` |
| `selectview` 権限 | `public.inquiries` への `SELECT`、`get_inquiries_by_email` への `EXECUTE` |
| `crudrole` 権限 | `public.inquiries` への `SELECT, INSERT, UPDATE, DELETE` |
| IAM 連携 | `select-function-lambda-role-*` と `crudrole-lambda-role-*` への `AWS IAM GRANT` |

現在の API 実装は `crudrole` ロールで DSQL に接続し、`inquiries` テーブルへ直接 `SELECT` / `INSERT` を実行します。`get_inquiries_by_email` 関数と `selectview` ロールは DB 側に残っていますが、現行の Lambda テンプレートからは使われていません。

## Liquibase 変更セット一覧

| 順番 | ファイル | 内容 |
| --- | --- | --- |
| 001 | `001_create_inquiries.sql` | `inquiries` テーブル作成 |
| 002 | `002_create_idx_inquiries_cognito_sub.sql` | `cognito_sub` インデックス追加 |
| 003 | `003_create_idx_inquiries_created_at.sql` | `created_at` インデックス追加 |
| 004 | `004_create_users.sql` | `users` テーブル作成 |
| 005 | `005_create_role_selectview.sql` | `selectview` ロール作成 |
| 006 | `006_create_role_crudrole.sql` | `crudrole` ロール作成 |
| 009 | `009_create_get_inquiries_by_email.sql` | メールアドレス検索関数作成 |
| 010 | `010_grant_function_selectview.sql` | `selectview` へ関数実行権限付与 |
| 011 | `011_grant_crudrole.sql` | `crudrole` へ `inquiries` の CRUD 権限付与 |
| 012 | `012_grant_selectview.sql` | `selectview` へ `inquiries` の SELECT 権限付与 |
| 015 | `015_aws_iam_grant_select-function-lambda-role-develop.sql` | `selectview` を develop IAM ロールへ付与 |
| 016 | `016_aws_iam_grant_select-function-lambda-role-main.sql` | `selectview` を main IAM ロールへ付与 |
| 017 | `017_aws_iam_grant_select-function-lambda-role-release.sql` | `selectview` を release IAM ロールへ付与 |
| 018 | `018_aws_iam_grant_crudrole-lambda-role-develop.sql` | `crudrole` を develop IAM ロールへ付与 |
| 019 | `019_aws_iam_grant_crudrole-lambda-role-main.sql` | `crudrole` を main IAM ロールへ付与 |
| 020 | `020_aws_iam_grant_crudrole-lambda-role-release.sql` | `crudrole` を release IAM ロールへ付与 |

`changelog.xml` に含まれる変更セットは上記です。`007`、`008`、`013`、`014` は現在のリポジトリに存在しません。

## コンテキスト

- `develop` / `main` / `release`: Aurora DSQL 向けの `CREATE INDEX ASYNC` と `AWS IAM GRANT` を含む変更を実行します。
- `local`: 通常の PostgreSQL で実行できる変更だけを適用し、Aurora DSQL 専用構文を避けます。API の CI 検証とローカル DB テストではこの `local` コンテキストを使います。

ローカル開発手順は [`../infrastructure/liquibase_migrate/README.md`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/infrastructure/liquibase_migrate/README.md) にあります。

## SeaORM 連携

`db_migrate.yaml` の `generate` ジョブは `sea-orm-cli generate entity` を実行し、`infrastructure/sea_orm/src/entity/` を更新します。差分があれば専用ブランチを作り、対象ブランチ向け Pull Request を自動作成します。

`inquiries.rs` には `reply` / `respondent` / `reply_at` を含む現在のテーブル定義が反映され、`users.rs` には `users` テーブル定義が反映されています。

## API との接続

- [API](api.md) は `${StackNamePrefix}-db-${Stage}-DSQLClusterEndpoint` を `Fn::ImportValue` で受け取り、`DSQL_ENDPOINT` 環境変数に設定します。
- Lambda は `create_db("crudrole", endpoint, region)` で接続し、JWT の `email` と `sub` を条件に `inquiries` を読み書きします。
- DB 権限は IAM ロール付与と DB ロール権限付与の両方で成立します。

## 関連ページ

- [README](README.md)
- [FAQ](FAQ.md)
- [API](api.md)
- [CI/CD](cicd.md)
