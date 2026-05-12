# nishidemasami.github.io-contactform ドキュメント

この Wiki は、`nishidemasami.github.io-contactform` リポジトリの **API / 認証 / データベース / CI/CD / FAQ** を横断して確認するための表紙です。Honkit の `README.md` として使う前提で、各ページへの入口と運用上の注意点をまとめています。

## 読み始め

- [目次](SUMMARY.md)
- [FAQ](FAQ.md)
- [API](api.md)
- [認証](auth.md)
- [データベース](db.md)
- [CI/CD](cicd.md)
- [FAQ](FAQ.md)

## 現在の構成

| 領域 | 現在の状態 | 主なソース |
| --- | --- | --- |
| API | `api/template.yaml` と `api/lambda/` で、Cognito JWT Authorizer 付き HTTP API と Rust Lambda を管理しています。 | [`../api/template.yaml`](../api/template.yaml), [`../api/lambda/src/`](../api/lambda/src/) |
| 認証 | `infrastructure/auth/template.yaml` で Cognito User Pool / User Pool Client を `develop` と `main` 向けにデプロイします。 | [`../infrastructure/auth/template.yaml`](../infrastructure/auth/template.yaml) |
| データベース | `infrastructure/liquibase_migrate/` で Aurora DSQL クラスターと Liquibase 変更セットを管理し、CI で SeaORM エンティティを再生成します。 | [`../infrastructure/liquibase_migrate/`](../infrastructure/liquibase_migrate/), [`../infrastructure/sea_orm/src/entity/`](../infrastructure/sea_orm/src/entity/) |
| CI/CD | GitHub Actions で API / 認証 / DB / ドキュメント配信 / Wiki 更新を自動化しています。 | [`../.github/workflows/`](../.github/workflows/) |
| フロント検証 | `testpage/` の静的 Next.js アプリが Cognito と API をつないで検証 UI を提供し、Cloudflare Pages に配信されます。 | [`../testpage/`](../testpage/) |

## ページ案内

- [FAQ](FAQ.md): 全体の通信経路や命名ルールなど、全ページにまたがる前提をまとめます。
- [API](api.md): `/inquiries` エンドポイント、Lambda 実装、認証・DB 依存を整理します。
- [認証](auth.md): Cognito のリソース構成、Outputs、API / testpage との接続点を整理します。
- [データベース](db.md): Aurora DSQL、Liquibase、DB ロール、SeaORM 生成フローを整理します。
- [CI/CD](cicd.md): GitHub Actions 各ワークフローの役割、トリガー、生成物、ドキュメント反映の流れを整理します。

## 更新時の着眼点

- API のエンドポイント、CORS、環境変数、OpenAPI 出力の扱いが変わったら [API](api.md) と [CI/CD](cicd.md) を一緒に更新します。
- Cognito の Outputs や JWT 前提を変えたら [認証](auth.md)、[API](api.md)、[FAQ](FAQ.md) の接続説明を見直します。
- DB スキーマ、権限、Export 名が変わったら [データベース](db.md) と API の依存説明を同期します。
- GitHub Actions を追加・変更したら [CI/CD](cicd.md) だけでなく、この表紙と [SUMMARY.md](SUMMARY.md) の導線も更新します。
- `document_cicd.yaml` は `docs/**` を監視していないため、**Wiki だけを更新しても公開ドキュメントは自動再配信されません**。必要に応じて手動実行や別のトリガーを考慮します。
