# 西出正美宛て問い合わせシステム ドキュメント

この Wiki は、`nishidemasami.github.io-contactform` リポジトリの **API / 認証 / データベース / CI/CD / FAQ / 更新ログ** を横断して確認するための表紙です。`docs/AGENTS.md` の運用方針に従い、Honkit の `README.md` として各ページへの入口と、ページ間の接続点を整理しています。

## 読み始め

- [目次](SUMMARY.md)
- [FAQ](FAQ.md)
- [API](api.md)
- [認証](auth.md)
- [データベース](db.md)
- [CI/CD](cicd.md)
- [AI開発](ai_development.md)
  - [タスクリスト (MoSCoW)](TASKS.md)
  - [WBS (アジャイル向け)](WBS.md)
- [更新ログ](log.md)

## 現在の構成

| 領域 | 現在の状態 | 主なソース |
| --- | --- | --- |
| API | `api/template.yaml` が HTTP API / Cognito JWT Authorizer / Rust Lambda を定義し、`api/lambda/` が `/inquiries` の GET / POST と OpenAPI 生成を実装します。 | [`api/template.yaml`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/api/template.yaml), [`api/lambda/src/`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/api/lambda/src/) |
| 認証 | `infrastructure/auth/template.yaml` が `develop` / `main` / `release` 向けの Cognito User Pool と User Pool Client を定義します。 | [`infrastructure/auth/template.yaml`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/infrastructure/auth/template.yaml) |
| データベース | `infrastructure/liquibase_migrate/` が Aurora DSQL クラスターと Liquibase 変更セットを管理し、`db_migrate.yaml` が SeaORM エンティティ生成まで自動化します。 | [`infrastructure/liquibase_migrate/`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/infrastructure/liquibase_migrate/), [`infrastructure/sea_orm/src/entity/`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/infrastructure/sea_orm/src/entity/) |
| CI/CD | GitHub Actions が API / 認証 / DB / ドキュメント配信 / Wiki 更新を担当し、OpenAPI・カバレッジ・SeaORM 生成物もリリースまたは PR として公開します。 | [`.github/workflows/`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/.github/workflows/) |
| フロント検証 | `testpage/` の静的 Next.js アプリが Cognito ログイン、問い合わせ一覧、問い合わせ作成の検証 UI を提供します。 | [`testpage/`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/testpage/) |

## ページ案内

- [FAQ](FAQ.md): 全体の接続関係、スタック名、公開フロー、OpenAPI の扱いをまとめます。
- [API](api.md): `/inquiries` の契約、Lambda 実装、OpenAPI 生成、認証・DB 依存を整理します。
- [認証](auth.md): Cognito のテンプレート、Outputs、API / testpage / OpenAPI との接続点を整理します。
- [データベース](db.md): Aurora DSQL、Liquibase、DB ロール、SeaORM 生成フローを整理します。
- [CI/CD](cicd.md): GitHub Actions 各ワークフローの役割、トリガー、生成物、Wiki 反映の流れを整理します。
- [AI開発](ai_development.md): AI が担当する開発タスクと、承認フロー内での利用箇所を整理します。
- [更新ログ](log.md): Wiki をいつ何の観点で更新したかを時系列で残します。

## 更新時の着眼点

- API のエンドポイント、JWT クレーム要件、CORS、OpenAPI 生成ロジックが変わったら [API](api.md) と [CI/CD](cicd.md) を一緒に更新します。
- Cognito の Outputs や App Client 前提が変わったら [認証](auth.md)、[API](api.md)、[FAQ](FAQ.md) の接続説明を見直します。
- DB スキーマ、権限、Export 名が変わったら [データベース](db.md) と [API](api.md) の依存説明を同期します。
- GitHub Actions を追加・変更したら [CI/CD](cicd.md) だけでなく、この表紙、[SUMMARY.md](SUMMARY.md)、必要なら [FAQ](FAQ.md) も更新します。
- `document_cicd.yaml` は `docs/**` を監視していないため、**Wiki だけを更新しても公開ドキュメントは自動再配信されません**。公開ページへの反映は `workflow_dispatch` か、`testpage/**` または関連ワークフローの変更時に限られます。
- `document_cicd.yaml` は Honkit ビルド直前に `docs/README.md` へ外部リンクを追記するため、**コミット済みの README と公開済みトップページの末尾リンクは一時的に一致しない**ことがあります。

## 品質担保・運用の要点

- **API 品質担保**: [CI/CD](cicd.md) の API ワークフローが `sam validate --lint`、ローカル PostgreSQL + Liquibase、`cargo check`、`cargo test -- --include-ignored`、`cargo tarpaulin`、`cargo run --features openapi --bin generate-openapi` を実行します。
- **フロント品質担保**: `document_cicd.yaml` が `testpage/` に対して `pnpm run lint`、`pnpm exec tsc --noEmit`、Storybook / TypeDoc / Next.js ビルドを実行します。
- **OpenAPI 運用**: API 契約は `api/lambda/src/bin/generate-openapi.rs` から生成され、`api/openapi.yaml` または `api/openapi-${branch}.yaml` として出力されます。詳細は [API](api.md) を参照してください。
- **Wiki 運用**: [AGENTS](AGENTS.md) をスキーマとして、索引（このページ / [SUMMARY](SUMMARY.md)）と相互参照を保ちながら `docs/` を継続更新します。

## リンク

- <a href="https://github.com/nishidemasami/nishidemasami.github.io-contactform" target="_blank">GitHub リポジトリ</a>
- <a href="https://ngicf-testpage.pages.dev/" target="_blank">開発者向けドキュメントページ（Cloudflare Pages）</a>
- <a href="https://nishidemasami.github.io/nishidemasami.github.io-contactform/" target="_blank">利用者向けドキュメントページ（GitHub Pages）</a>
<!-- この行以降は自動でリンクが挿入されるので、編集や追記をしないでください。 -->
