# データベース

## 概要

データベース基盤は `infrastructure/liquibase_migrate/` と `infrastructure/sea_orm/` で管理されています。AWS SAM で **AWS DSQL クラスター** を作成し、Liquibase でスキーマを適用し、その結果をもとに SeaORM エンティティを生成する流れです。

- テンプレート: [`../infrastructure/liquibase_migrate/template.yaml`](../infrastructure/liquibase_migrate/template.yaml)
- 変更管理: [`../infrastructure/liquibase_migrate/changelog.xml`](../infrastructure/liquibase_migrate/changelog.xml)
- 生成物: [`../infrastructure/sea_orm/src/entity/`](../infrastructure/sea_orm/src/entity/)
- 自動化: [CI/CD](cicd.md)

## DSQL クラスター

| 項目 | 内容 |
| --- | --- |
| リソース | `AWS::DSQL::Cluster` |
| Stage | `develop`, `main` |
| 主な Output | `DSQLClusterIdentifier`, `DSQLClusterEndpoint` |
| Export 名 | `${StackNamePrefix}-app-db-${Stage}-DSQLClusterIdentifier`, `${StackNamePrefix}-app-db-${Stage}-DSQLClusterEndpoint` |

`db_migrate.yaml` は、デプロイしたスタック `nishidemasami-github-io-contactform-db-${branch}` から endpoint を取得し、Liquibase と SeaORM 生成に再利用します。

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
| ロール | `selectview` |
| 権限 | `selectview` に関数実行権限、および `public` スキーマの全テーブルへの `SELECT, INSERT, UPDATE` を付与 |
| IAM 連携 | `arn:aws:iam::672530906129:role/select-function-lambda-role` に `selectview` を付与 |

現在の API 実装は `selectview` ロールで DSQL に接続し、`inquiries` テーブルへ直接 `SELECT` / `INSERT` を実行します。`get_inquiries_by_email` 関数は DB 側に存在しますが、Rust ハンドラーからはまだ使っていません。

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
| 011 | `011_aws_iam_grant_select-function-lambda-role.sql` | IAM ロールへ DB ロール付与 |
| 012 | `012_grant_selectview.sql` | テーブル権限付与 |

`changelog.xml` に含まれる変更セットは上記のみで、`006`〜`008` は現時点で未使用です。

## SeaORM 連携

`db_migrate.yaml` の `generate` ジョブは `sea-orm-cli generate entity` を実行し、`infrastructure/sea_orm/src/entity/` を更新します。現在のリポジトリには少なくとも次の生成物が含まれています。

- `inquiries.rs`
- `users.rs`
- `mod.rs`
- `prelude.rs`

Wiki 側では「生成済みエンティティが未収録」とはせず、**生成ジョブと生成物が両方存在する状態**として扱う必要があります。

## API との接続メモ

`api/template.yaml` は `DSQL_ENDPOINT` に `${StackNamePrefix}-db-${Stage}-DSQLClusterEndpoint` を Import する構成ですが、DB テンプレートの Export 名は `${StackNamePrefix}-app-db-${Stage}-DSQLClusterEndpoint` です。ドキュメント上も、現在のソースには **DB Export 名と API Import 名の差異がある** 状態として記録しておきます。

## 関連ページ

- [API](api.md)
- [認証](auth.md)
- [CI/CD](cicd.md)
