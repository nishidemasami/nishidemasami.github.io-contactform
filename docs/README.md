# nishidemasami.github.io-contactform ドキュメント

このドキュメントは、`nishidemasami.github.io-contactform` リポジトリの構成と運用を俯瞰するための Wiki です。現在のリポジトリには主に **認証基盤**、**データベース基盤**、**GitHub Actions による CI/CD** が含まれており、API 実装本体はまだ同梱されていません。

## 読み始め

- [目次](SUMMARY.md)
- [API](api.md)
- [認証](auth.md)
- [データベース](db.md)
- [CI/CD](cicd.md)

## いま分かること

| 領域 | 概要 | 主なソース |
| --- | --- | --- |
| API | `/api/` ディレクトリは未配置で、実装は未収録です。 | なし |
| 認証 | AWS SAM で Cognito User Pool と App Client をデプロイします。 | [`../infrastructure/auth/template.yaml`](../infrastructure/auth/template.yaml) |
| データベース | AWS DSQL クラスターを作成し、Liquibase でスキーマを適用します。 | [`../infrastructure/liquibase_migrate/`](../infrastructure/liquibase_migrate/) |
| CI/CD | GitHub Actions で認証・DB・Wiki 更新を自動化します。 | [`../.github/workflows/`](../.github/workflows/) |

## ページ構成

- [API](api.md): 現在の API 実装状況と、認証・DB との接続前提を整理します。
- [認証](auth.md): Cognito 構成、出力値、デプロイ経路をまとめます。
- [データベース](db.md): DSQL、Liquibase、権限、SeaORM 生成までをまとめます。
- [CI/CD](cicd.md): 各ワークフローのトリガー、役割、依存関係をまとめます。

## 更新方針

- 新しい API 実装が `api/` 配下に追加されたら、まず [API](api.md) を更新します。
- 認証や DB のインフラ変更が入ったら、関連ページから相互リンクを張り直します。
- GitHub Actions を変更した場合は、[CI/CD](cicd.md) のトリガー条件とデプロイ対象を同期します。
