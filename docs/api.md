# API

## 概要

API は `api/template.yaml` と `api/lambda/` で管理されています。AWS SAM テンプレートが **HTTP API Gateway / Cognito JWT Authorizer / Rust Lambda / Lambda 実行ロール** を定義し、Rust 側は `main.rs` から `handlers.rs` と `db.rs` を呼び出して `/inquiries` を処理します。

- SAM テンプレート: [`../api/template.yaml`](../api/template.yaml)
- Lambda エントリーポイント: [`../api/lambda/src/main.rs`](../api/lambda/src/main.rs)
- ハンドラー: [`../api/lambda/src/handlers.rs`](../api/lambda/src/handlers.rs)
- DB 接続: [`../api/lambda/src/db.rs`](../api/lambda/src/db.rs)
- レスポンス / リクエスト型: [`../api/lambda/src/models.rs`](../api/lambda/src/models.rs)
- 関連デプロイ: [CI/CD](cicd.md)

## インフラ構成

| 要素 | 内容 |
| --- | --- |
| API Gateway | `AWS::Serverless::HttpApi` |
| 認可 | Cognito JWT Authorizer（既定 Authorizer） |
| Lambda Runtime | `provided.al2023` |
| アーキテクチャ | `arm64` |
| メモリ / タイムアウト | `128 MB` / `30 秒` |
| 実行ロール | `select-function-lambda-role-${Stage}` |

Lambda の IAM ロールには `dsql:DbConnect` 権限があり、DB 側では [データベース](db.md) の `selectview` ロール付与で実アクセスを成立させます。

## 公開エンドポイント

現在の HTTP API は `/inquiries` のみを公開しています。

| メソッド | パス | 認証 | 処理 |
| --- | --- | --- | --- |
| `GET` | `/inquiries` | 必須 | JWT の `email` と `sub` を使い、自分の問い合わせ一覧を `created_at DESC` で返します。 |
| `POST` | `/inquiries` | 必須 | JWT の `email` と `sub` を使い、新しい問い合わせを `inquiries` テーブルへ登録します。 |

`PUT` / `DELETE` などのルートは定義されておらず、Lambda 側は未対応メソッドに `405 Method Not Allowed` を返します。

## 認証と CORS

API Gateway は Cognito JWT Authorizer を既定の認可方式にし、次の値を [認証](auth.md) の CloudFormation Export から読み込みます。

- Issuer: `${StackNamePrefix}-auth-${Stage}-CognitoIssuer`
- Audience: `${StackNamePrefix}-auth-${Stage}-CognitoUserPoolClientId`

Lambda 側でも JWT クレームを追加検査し、次の条件を満たさない場合は `401 Unauthorized` を返します。

- `email` が存在し、空文字でないこと
- `sub` が存在し、UUID として解釈できること

CORS は SAM テンプレートの `StageCorsMap` で切り替えています。

| Stage | `AllowOrigin` |
| --- | --- |
| `develop` | `https://ngicf-testpage.pages.dev` |
| `main` | `https://nishidemasami.github.io` |

Lambda のレスポンスでも `Access-Control-Allow-Origin` と `Content-Type: application/json` を明示的に返します。

## リクエスト / レスポンス

### `GET /inquiries`

成功時は `200 OK` で次の JSON を返します。

| フィールド | 内容 |
| --- | --- |
| `email` | JWT から取得したメールアドレス |
| `count` | 取得件数 |
| `inquiries` | `id`, `cognito_sub`, `email`, `subject`, `body`, `created_at` の配列 |

検索条件は `email` と `cognito_sub` の両方です。DB には `get_inquiries_by_email` 関数もありますが、現在の Rust 実装は生 SQL で `inquiries` テーブルを直接参照しています。

### `POST /inquiries`

リクエスト本文は次の JSON です。

```json
{
  "subject": "件名",
  "body": "本文"
}
```

成功時は `201 Created` を返し、保存した 1 件を `inquiry` フィールドに包みます。`id` は UUID v7、`created_at` は Lambda 実行時刻です。

## 実行時設定

| 環境変数 | 用途 | 設定元 |
| --- | --- | --- |
| `DSQL_ENDPOINT` | Aurora DSQL 接続先 | [データベース](db.md) スタックの `DSQLClusterEndpoint` Export |
| `DSQL_REGION` | DSQL 接続リージョン | SAM テンプレート固定値 `ap-northeast-3` |
| `CORS_ORIGIN` | `Access-Control-Allow-Origin` | `StageCorsMap` |

Rust 側は `create_db("selectview", endpoint, region)` で接続文字列を組み立て、Aurora DSQL SQLx connector 経由で SeaORM 接続を作成します。

## エラー動作

| 条件 | ステータス | 備考 |
| --- | --- | --- |
| JWT の `email` / `sub` が欠落、または `sub` が UUID でない | `401` | Lambda 側の追加検査で拒否します。 |
| 未対応メソッド | `405` | Lambda 側で明示的に返します。 |
| DB 接続失敗、JSON 解析失敗、SELECT / INSERT 失敗など | `500` | ログ出力後に共通の内部エラーレスポンスを返します。 |

## 出力値と依存関係

SAM テンプレートは次の Outputs を公開します。

| Output | 用途 |
| --- | --- |
| `HttpApiUrl` | `https://${HttpApi}.execute-api.ap-northeast-3.amazonaws.com` を Export し、`testpage` ビルドなどで参照します。 |
| `HttpApiId` | OpenAPI エクスポート時に API ID を取得するために使います。 |

主な依存関係は次の通りです。

| 依存先 | API 側で使うもの | 参照 |
| --- | --- | --- |
| 認証 | Cognito Issuer / User Pool Client ID | [認証](auth.md) |
| DB | `inquiries` テーブル、`selectview` ロール、`DSQLClusterEndpoint` | [データベース](db.md) |
| CI/CD | `cargo check` / `cargo test`、SAM デプロイ、OpenAPI エクスポート | [CI/CD](cicd.md) |

## OpenAPI について

`api_cicd.yaml` の `export_openapi` ジョブは、デプロイ済み API Gateway から `api/openapi.yaml` をエクスポートして差分があればコミットします。つまり OpenAPI は手書きではなく、**デプロイ後の API Gateway 定義を CI が取り込む** フローです。

## 関連ページ

- [FAQ](FAQ.md)
- [認証](auth.md)
- [データベース](db.md)
- [CI/CD](cicd.md)
