# nishidemasami.github.io-contactform ドキュメント

この Wiki は、`nishidemasami.github.io-contactform` リポジトリの **API / 認証 / データベース / CI/CD / FAQ / 更新ログ** を横断して確認するための表紙です。Honkit の `README.md` として使う前提で、各ページへの入口と更新時に見直すべき接続点をまとめています。

## 読み始め

- [目次](SUMMARY.md)
- [FAQ](FAQ.md)
- [API](api.md)
- [認証](auth.md)
- [データベース](db.md)
- [CI/CD](cicd.md)
- [更新ログ](log.md)

## 現在の構成

| 領域 | 現在の状態 | 主なソース |
| --- | --- | --- |
| API | `api/template.yaml` と `api/lambda/` で、Cognito JWT Authorizer 付き HTTP API と Rust Lambda を管理しています。Lambda は `crudrole` で Aurora DSQL に接続し、`/inquiries` の GET / POST を処理します。 | [`api/template.yaml`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/api/template.yaml), [`api/lambda/src/`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/api/lambda/src/) |
| 認証 | `infrastructure/auth/template.yaml` で Cognito User Pool / User Pool Client を `develop` / `main` / `release` 向けに定義しています。 | [`infrastructure/auth/template.yaml`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/infrastructure/auth/template.yaml) |
| データベース | `infrastructure/liquibase_migrate/` で Aurora DSQL クラスター、Liquibase 変更セット、IAM ロール連携を管理し、CI で SeaORM エンティティを再生成します。 | [`infrastructure/liquibase_migrate/`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/infrastructure/liquibase_migrate/), [`infrastructure/sea_orm/src/entity/`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/infrastructure/sea_orm/src/entity/) |
| CI/CD | GitHub Actions で API / 認証 / DB / ドキュメント配信 / Wiki 更新を自動化しています。自動デプロイの主対象は `develop` / `release` で、`main` は主に PR 検証または手動実行で扱います。 | [`.github/workflows/`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/.github/workflows/) |
| フロント検証 | `testpage/` の静的 Next.js アプリが Cognito と API をつなぐ検証 UI を提供し、Cloudflare Pages に配信されます。 | [`testpage/`](https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/testpage/) |

## ページ案内

- [FAQ](FAQ.md): 全体の通信経路、命名規則、公開方法の前提をまとめます。
- [API](api.md): `/inquiries` エンドポイント、Lambda 実装、認証・DB 依存を整理します。
- [認証](auth.md): Cognito のリソース構成、Outputs、API / testpage との接続点を整理します。
- [データベース](db.md): Aurora DSQL、Liquibase、DB ロール、SeaORM 生成フローを整理します。
- [CI/CD](cicd.md): GitHub Actions 各ワークフローの役割、トリガー、生成物、ドキュメント反映の流れを整理します。
- [更新ログ](log.md): Wiki をいつ何の観点で更新したかを時系列で残します。

## 更新時の着眼点

- API のエンドポイント、CORS、環境変数、OpenAPI 出力名が変わったら [API](api.md) と [CI/CD](cicd.md) を一緒に更新します。
- Cognito の Outputs、Stage、JWT 前提を変えたら [認証](auth.md)、[API](api.md)、[FAQ](FAQ.md) の接続説明を見直します。
- DB スキーマ、権限、Export 名が変わったら [データベース](db.md) と [API](api.md) の依存説明を同期します。
- GitHub Actions を追加・変更したら [CI/CD](cicd.md) だけでなく、この表紙、[SUMMARY.md](SUMMARY.md)、必要なら [FAQ](FAQ.md) も更新します。
- `document_cicd.yaml` は `docs/**` を監視していないため、**Wiki だけを更新しても公開ドキュメントは自動再配信されません**。公開ページへの反映は `workflow_dispatch` か `testpage/**` / ワークフローファイル変更時の実行に依存します。

## 品質担保・開発運用の要点

- **品質担保**: [CI/CD](cicd.md) で `cargo run --features openapi --bin generate-openapi` を検証し、Rust（`utoipa`）を OpenAPI の SSoT として扱います。DB は [データベース](db.md) の Liquibase 変更セットを CI の local PostgreSQL でも適用して環境差異を抑制します。Rust 側は `proptest` によるプロパティベーステスト、`cargo clippy` による静的解析、言語仕様に基づくメモリ安全 / NULL 安全 / 型安全を前提に品質を担保します。
- **GitFlow とレビュー**: AI は `copilot/**` ブランチで変更を作成し `develop` へ PR を出し、人間レビューで CI/CD 結果を確認します。その後 `develop -> main -> release` の PR フローで開発線と保守線を明確に保ちます。詳細は [CI/CD](cicd.md) を参照してください。
- **AWS の活用方針**: 24/365 保守が重い EC2 などを避け、Lambda / Aurora DSQL などのマネージド・スケーラブルな従量課金サービスを中心に構成することで、Scale to Zero を含むクラウドの恩恵を最大化します（[API](api.md), [データベース](db.md)）。
- **llm-wiki 運用**: [AGENTS](AGENTS.md) をスキーマとして `docs/` を AI が読みやすい構造で継続更新し、索引（このページ / [SUMMARY](SUMMARY.md)）と相互参照を維持します。

## リンク

- <a href="https://github.com/nishidemasami/nishidemasami.github.io-contactform" target="_blank">GitHubリポジトリ</a>
- <a href="https://ngicf-testpage.pages.dev/" target="_blank">開発者向けドキュメントページ（CloudFlare Pages）</a>
- <a href="https://nishidemasami.github.io/nishidemasami.github.io-contactform/" target="_blank">利用者向けドキュメントページ（GitHub Pages）</a>
<!-- この行以降は自動でリンクが挿入されるので、編集や追記をしないでください。 -->
