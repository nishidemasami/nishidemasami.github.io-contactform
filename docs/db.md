# データベース

## 概要

データベース基盤は `infrastructure/liquibase_migrate/` で管理されています。AWS SAM で **AWS DSQL クラスター** を作成し、その後 Liquibase でスキーマを適用し、最後に SeaORM エンティティ生成まで自動化します。

- テンプレート: [`../infrastructure/liquibase_migrate/template.yaml`](../infrastructure/liquibase_migrate/template.yaml)
- 変更管理: [`../infrastructure/liquibase_migrate/changelog.xml`](../infrastructure/liquibase_migrate/changelog.xml)
- 変更セット群: [`../infrastructure/liquibase_migrate/changes/`](../infrastructure/liquibase_migrate/changes/)
- 自動化: [CI/CD](cicd.md)

## インフラ

| 項目 | 内容 |
| --- | --- |
| リソース | `AWS::DSQL::Cluster` |
| Stage | `develop`, `main` |
| 主な Output | `DSQLClusterIdentifier`, `DSQLClusterEndpoint` |

`DSQLClusterEndpoint` は、Liquibase 実行時と SeaORM エンティティ生成時の接続先として再利用されます。

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
| `idx_inquiries_created_at` | `inquiries(created_at)` | 時系列検索の高速化 |

### 関数と権限

| 要素 | 内容 |
| --- | --- |
| 関数 | `get_inquiries_by_email(p_email VARCHAR(255))` |
| ロール | `selectview` |
| 権限 | `selectview` に関数実行権限、および `public` スキーマの全テーブルへの `SELECT, INSERT, UPDATE` を付与 |
| IAM 連携 | `arn:aws:iam::779854054594:role/select-function-lambda-role` に `selectview` を付与 |

`inquiries.cognito_sub` があるため、問い合わせデータは Cognito ユーザーと関連づく想定です。認証基盤との接続前提は [認証](auth.md) を参照してください。

## Liquibase 変更セット一覧

| 順番 | ファイル | 内容 |
| --- | --- | --- |
| 001 | `001_create_inquiries.sql` | `inquiries` テーブル作成 |
| 002 | `002_create_idx_inquiries_cognito_sub.sql` | `cognito_sub` インデックス追加 |
| 003 | `003_create_idx_inquiries_created_at.sql` | `created_at` インデックス追加 |
| 004 | `004_create_users.sql` | `users` テーブル作成 |
| 005 | `005_create_role_selectview.sql` | `selectview` ロール作成 |
| 009 | `009_create_get_inquiries_by_email.sql` | メール検索関数作成 |
| 010 | `010_grant_function_selectview.sql` | 関数実行権限付与 |
| 011 | `011_aws_iam_grant_select-function-lambda-role.sql` | IAM ロールへ DB ロール付与 |
| 012 | `012_grant_selectview.sql` | テーブル権限付与 |

番号 `006`〜`008` は現時点の `changelog.xml` に含まれていません。新しい変更セットを追加する場合は、番号体系と `changelog.xml` の include 順序を合わせて管理します。

## SeaORM 連携

GitHub Actions では、マイグレーション完了後に `sea-orm-cli` を使って `infrastructure/sea_orm/src/entity` へエンティティを生成する構成です。ただし、現在のリポジトリには生成済みエンティティはまだ含まれていません。

- ワークスペース: [`../infrastructure/sea_orm/`](../infrastructure/sea_orm/)
- パッケージ定義: [`../infrastructure/sea_orm/Cargo.toml`](../infrastructure/sea_orm/Cargo.toml)

## 関連ページ

- [API](api.md)
- [認証](auth.md)
- [CI/CD](cicd.md)
