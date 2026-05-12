# nishidemasami.github.io-contactform ドキュメント

この Wiki は、`nishidemasami.github.io-contactform` リポジトリの **API / 認証 / データベース / CI/CD** を俯瞰するための入口です。Honkit の表紙として使う前提で、各トピックへの導線と現在の実装状況をまとめています。

## 読み始め

- [目次](SUMMARY.md)
- [API](api.md)
- [認証](auth.md)
- [データベース](db.md)
- [CI/CD](cicd.md)
- [FAQ](FAQ.md)

## 現在の構成

| 領域 | 現在の状態 | 主なソース |
| --- | --- | --- |
| API | `api/template.yaml` と `api/lambda/` で、Cognito JWT 認証付きの HTTP API と Rust Lambda を管理しています。 | [`../api/template.yaml`](../api/template.yaml), [`../api/lambda/src/`](../api/lambda/src/) |
| 認証 | Cognito User Pool / User Pool Client を AWS SAM でデプロイします。 | [`../infrastructure/auth/template.yaml`](../infrastructure/auth/template.yaml) |
| データベース | AWS DSQL クラスター、Liquibase 変更セット、SeaORM エンティティ生成を管理しています。 | [`../infrastructure/liquibase_migrate/`](../infrastructure/liquibase_migrate/), [`../infrastructure/sea_orm/`](../infrastructure/sea_orm/) |
| CI/CD | GitHub Actions で API / 認証 / DB / ドキュメント / Wiki 更新を自動化しています。 | [`../.github/workflows/`](../.github/workflows/) |
| フロント検証 | `testpage/` で Cognito と API を接続する Next.js 検証ページを管理しています。 | [`../testpage/`](../testpage/) |

## ページ案内

- [API](api.md): `/inquiries` エンドポイント、Lambda の処理、認証・DB 依存を整理します。
- [認証](auth.md): Cognito のリソース構成、出力値、API 側との接続点を整理します。
- [データベース](db.md): DSQL、Liquibase、DB ロール、SeaORM 生成物を整理します。
- [CI/CD](cicd.md): GitHub Actions 各ワークフローの役割、トリガー、生成物を整理します。

## 更新方針

- API のエンドポイント、レスポンス、環境変数、OpenAPI 出力の扱いが変わったら [API](api.md) と [CI/CD](cicd.md) を一緒に更新します。
- Cognito の Output や JWT 前提を変えたら [認証](auth.md) と API/CI の参照先を見直します。
- DB スキーマや権限が変わったら [データベース](db.md) に加え、API 実装が依存する箇所も確認します。
- ワークフローを追加・変更したら [CI/CD](cicd.md) とこの表紙の概要表を同期します。
