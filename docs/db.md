# データベース

## 概要

データベース基盤は `infrastructure/liquibase_migrate/` で管理されています。AWS SAM で **Aurora DSQL クラスター** を作成し、Liquibase でスキーマを適用し、その結果をもとに GitHub Actions が `infrastructure/sea_orm/src/entity/` を再生成します。

- テンプレート: [`../infrastructure/liquibase_migrate/template.yaml`](../infrastructure/liquibase_migrate/template.yaml)
- 変更管理: [`../infrastructure/liquibase_migrate/changelog.xml`](../infrastructure/liquibase_migrate/changelog.xml)
- 補足 README: [`../infrastructure/liquibase_migrate/README.md`](../infrastructure/liquibase_migrate/README.md)
- SeaORM 生成物: [`../infrastructure/sea_orm/src/entity/`](../infrastructure/sea_orm/src/entity/)
- 自動化: [CI/CD](cicd.md)

## DSQL クラスター

| 項目 | 内容 |
| --- | --- |
| リソース | `AWS::DSQL::Cluster` |
| Stage | `develop`, `main` |
| 主な Outputs | `DSQLClusterIdentifier`, `DSQLClusterEndpoint` |
| Export 名 | `${StackNamePrefix}-db-${Stage}-DSQLClusterIdentifier`, `${StackNamePrefix}-db-${Stage}-DSQLClusterEndpoint` |

`db_migrate.yaml` は `snngicf-db-${branch}` スタックをデプロイし、CloudFormation 出力から endpoint を取得して Liquibase と SeaORM 生成に再利用します。[API](api.md) 側の `DSQL_ENDPOINT` Import 名もこの Export と一致しています。

## スキーマ

### テーブル

| テーブル | 主なカラム | 用途 |
| --- | --- | --- |
| `inquiries` | `id`, `cognito_sub`, `email`, `subject`, `body`, `created_at` | 問い合わせ保存 |
| `users` | `id`, `email`, `username`, `hashed_password`, `created_at` | ユーザー情報保存 |

### インデックス

| インデックス | 対象 | 目的 |
| --- | --- | --- |
| `idx_inquiries_cognito_sub` | `inquiries(cognito_sub)` | ユーザー単位の検索高速化 |
| `idx_inquiries_created_at` | `inquiries(created_at)` | 新しい問い合わせ順の取得を補助 |

### 関数と権限

| 要素 | 内容 |
| --- | --- |
| 関数 | `get_inquiries_by_email(p_email VARCHAR(255))` |
| DB ロール | `selectview WITH LOGIN` |
| テーブル権限 | `public` スキーマの全テーブルへ `SELECT, INSERT, UPDATE` |
| 関数権限 | `get_inquiries_by_email` への `EXECUTE` |
| IAM 連携 | `select-function-lambda-role-develop` / `select-function-lambda-role-main` に `selectview` を付与 |

現在の API 実装は `selectview` ロールで DSQL に接続し、`inquiries` テーブルへ直接 `SELECT` / `INSERT` を実行します。`get_inquiries_by_email` 関数は DB 側にありますが、Rust ハンドラーからは呼ばれていません。

## Liquibase 変更セット一覧

| 順番 | ファイル | 内容 |
| --- | --- | --- |
| 001 | `001_create_inquiries.sql` | `inquiries` テーブル作成 |
| 002 | `002_create_idx_inquiries_cognito_sub.sql` | `cognito_sub` インデックス追加 |
| 003 | `003_create_idx_inquiries_created_at.sql` | `created_at` インデックス追加 |
| 004 | `004_create_users.sql` | `users` テーブル作成 |
| 005 | `005_create_role_selectview.sql` | `selectview` ロール作成 |
| 009 | `009_create_get_inquiries_by_email.sql` | メールアドレス検索関数作成 |
| 010 | `010_grant_function_selectview.sql` | 関数実行権限付与 |
| 012 | `012_grant_selectview.sql` | テーブル権限付与 |
| 015 | `015_aws_iam_grant_select-function-lambda-role-develop.sql` | IAM ロールへ DB ロール付与（develop） |
| 016 | `016_aws_iam_grant_select-function-lambda-role-main.sql` | IAM ロールへ DB ロール付与（main） |

`changelog.xml` に含まれる変更セットは上記です。`006`〜`008`、`011`、`013`、`014` は現在のリポジトリに存在しません。

## コンテキスト

- `main` / `develop`: Aurora DSQL 向けの `CREATE INDEX ASYNC` と `AWS IAM GRANT` を含む変更を実行します。
- `local`: 通常の PostgreSQL で実行できる `CREATE INDEX` のみを使い、Aurora DSQL 専用構文を避けます。

ローカル開発手順は [`../infrastructure/liquibase_migrate/README.md`](../infrastructure/liquibase_migrate/README.md) にあります。

## SeaORM 連携

`db_migrate.yaml` の `generate` ジョブは `sea-orm-cli generate entity` を実行し、`infrastructure/sea_orm/src/entity/` を更新します。現在のリポジトリには少なくとも次の生成物があります。

- `inquiries.rs`
- `users.rs`
- `mod.rs`
- `prelude.rs`

Wiki 側では、**Liquibase 変更セットと SeaORM 生成物が CI で同期される構成**として扱うのが現在の実態です。

## API との接続

- [API](api.md) は `${StackNamePrefix}-db-${Stage}-DSQLClusterEndpoint` を `Fn::ImportValue` で受け取り、`DSQL_ENDPOINT` 環境変数に設定します。
- Lambda は `create_db("selectview", endpoint, region)` で接続し、JWT の `email` と `sub` を条件に `inquiries` を読み書きします。
- DB 権限は IAM ロール付与と DB ロール権限付与の両方で成立します。

## 関連ページ

- [FAQ](FAQ.md)
- [API](api.md)
- [CI/CD](cicd.md)
