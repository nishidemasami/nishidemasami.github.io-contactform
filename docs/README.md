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
| API | `api/template.yaml` と `api/lambda/` で、Cognito JWT Authorizer 付き HTTP API と Rust Lambda を管理しています。Lambda は `crudrole` で Aurora DSQL に接続し、`/inquiries` の GET / POST を処理します。 | [`../api/template.yaml`](../api/template.yaml), [`../api/lambda/src/`](../api/lambda/src/) |
| 認証 | `infrastructure/auth/template.yaml` で Cognito User Pool / User Pool Client を `develop` / `main` / `release` 向けに定義しています。 | [`../infrastructure/auth/template.yaml`](../infrastructure/auth/template.yaml) |
| データベース | `infrastructure/liquibase_migrate/` で Aurora DSQL クラスター、Liquibase 変更セット、IAM ロール連携を管理し、CI で SeaORM エンティティを再生成します。 | [`../infrastructure/liquibase_migrate/`](../infrastructure/liquibase_migrate/), [`../infrastructure/sea_orm/src/entity/`](../infrastructure/sea_orm/src/entity/) |
| CI/CD | GitHub Actions で API / 認証 / DB / ドキュメント配信 / Wiki 更新を自動化しています。自動デプロイの主対象は `develop` / `release` で、`main` は主に PR 検証または手動実行で扱います。 | [`../.github/workflows/`](../.github/workflows/) |
| フロント検証 | `testpage/` の静的 Next.js アプリが Cognito と API をつなぐ検証 UI を提供し、Cloudflare Pages に配信されます。 | [`../testpage/`](../testpage/) |

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
