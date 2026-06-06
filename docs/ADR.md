# Architecture Decision Records (ADR)

このドキュメントは、本プロジェクトにおいて下された重要なアーキテクチャ上の決定とその背景を記録するものです。AI エージェントがコードベースを理解し、一貫したアプローチで開発を進めるための道標となります。

## ADR 1: OpenAPI の生成方法について

**決定事項**: API Gateway から OpenAPI の定義をエクスポートするのではなく、Rust コード (`utoipa` クレート) を用いてコードベースから OpenAPI 定義を生成する。

**背景・理由**:
- **単一の真実の源 (Single Source of Truth)**: API の振る舞い、リクエスト・レスポンスの型は Rust のコード (`api/lambda/src/handlers.rs`, `models.rs`) に最も正確に表現されています。これを正として OpenAPI を生成することで、実装とドキュメントの乖離を防ぐことができます。
- **型安全と保守性**: `utoipa` を使うことで、Rust の強力な型システムと連携したドキュメント生成が可能になります。
- **CI/CDとの統合**: GitHub Actions で `generate-openapi.rs` を実行し、自動的に RapiDoc, Swagger UI などの公開ドキュメントに反映できるため、デベロッパー体験が向上します。

## ADR 2: データベースのマイグレーションと ORM について

**決定事項**:
1. データベースのスキーマ管理およびマイグレーションには **Liquibase** を使用する。
2. アプリケーションコードからのデータベース操作には **SeaORM** を使用し、エンティティは Liquibase のスキーマから自動生成する。

**背景・理由**:
- **スキーマ管理の独立性**: Aurora DSQL を対象とした堅牢なスキーマ進化を管理するため、Rust に依存しないツールである Liquibase を採用しています。
- **SeaORM の自動生成**: `infrastructure/sea_orm/src/entity/` 内のコードは `db_migrate.yaml` によって Liquibase で構成された PostgreSQL (ローカル) から自動生成されます。これにより、手動でエンティティを記述する際のエラーを排除できます。
- **ルール**: 開発者（AI 含む）は SeaORM のエンティティファイルを直接編集してはいけません。必ず Liquibase のチェンジセット (`infrastructure/liquibase_migrate/changes/`) を追加・編集してください。

## ADR 3: インフラストラクチャとしての AWS SAM の採用

**決定事項**: AWS リソース (Lambda, API Gateway, Cognito, IAM ロールなど) の定義とデプロイには **AWS SAM (Serverless Application Model)** を使用する。

**背景・理由**:
- サーバーレスアーキテクチャの構築において、CloudFormation の冗長な記述を簡略化できるため。
- `sam build` や `sam deploy` による強力なローカル開発・デプロイ体験が得られるため。
- Rust ベースの Lambda (`cargo-lambda`) との統合が容易であるため (`Metadata.BuildMethod: rust-cargolambda`)。

## ADR 4: フロントエンドのホスティングと技術スタック

**決定事項**: フロントエンド検証環境 (`testpage/`) は **Next.js** で構築し、**Cloudflare Pages** に静的エクスポート (`next build && next export` 相当の `output: "export"`) してデプロイする。

**背景・理由**:
- 検証用 UI としては静的なファイル配信で十分であり、サーバーサイドレンダリング (SSR) 用のインフラを AWS 内に別途構築・維持するコストを削減するため。
- GitHub Actions と Cloudflare Pages の連携がシンプルかつ高速であるため。
- Cognito によるクライアントサイド認証 (`aws-amplify`) と相性が良いため。
