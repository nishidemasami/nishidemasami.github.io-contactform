# API

## 概要

API は `api/template.yaml` と `api/lambda/` で管理されています。AWS SAM テンプレートが **HTTP API Gateway / Cognito JWT Authorizer / Rust Lambda / Lambda 実行ロール** をまとめて定義し、実処理は `api/lambda/src/main.rs` から `handlers.rs` と `db.rs` を呼び出します。

- SAM テンプレート: [`../api/template.yaml`](../api/template.yaml)
- Lambda 実装: [`../api/lambda/src/main.rs`](../api/lambda/src/main.rs)
- ハンドラー: [`../api/lambda/src/handlers.rs`](../api/lambda/src/handlers.rs)
- モデル: [`../api/lambda/src/models.rs`](../api/lambda/src/models.rs)
- デプロイ: [CI/CD](cicd.md)

## 公開エンドポイント

現在の HTTP API は `/inquiries` のみを公開しています。

| メソッド | パス | 認証 | 処理 |
| --- | --- | --- | --- |
| `GET` | `/inquiries` | 必須 | JWT の `email` と `sub` を使って、自分の問い合わせ一覧を新しい順に返します。 |
| `POST` | `/inquiries` | 必須 | JWT の `email` と `sub` を使って、新しい問い合わせを `inquiries` テーブルへ登録します。 |

他のメソッドは Lambda 側で `405 Method Not Allowed` を返します。

## 認証とリクエスト前提

API Gateway は Cognito JWT Authorizer を既定の認可方式として使います。Lambda でも追加でクレーム検査を行い、次の値が揃わない場合は `401 Unauthorized` を返します。

- `email`
- `sub`（Lambda 内では UUID として解釈）

Issuer と Audience は [認証](auth.md) の CloudFormation Export を `Fn::ImportValue` で取り込みます。

## リクエスト / レスポンス

### `GET /inquiries`

成功時は `200 OK` で次の JSON を返します。

| フィールド | 内容 |
| --- | --- |
| `email` | JWT から取得したメールアドレス |
| `count` | 取得件数 |
| `inquiries` | `id`, `cognito_sub`, `email`, `subject`, `body`, `created_at` の配列 |

検索条件は `email` と `cognito_sub` の両方です。DB 上に `get_inquiries_by_email` 関数はありますが、現在の Rust 実装は関数ではなく SQL を直接発行しています。

### `POST /inquiries`

リクエスト本文は次の JSON です。

```json
{
  "subject": "件名",
  "body": "本文"
}
```

成功時は `201 Created` で、保存した 1 件を `inquiry` フィールドに包んで返します。`id` は UUID v7、`created_at` は Lambda 実行時刻で生成されます。

## 実行時設定

| 環境変数 | 用途 | 設定元 |
| --- | --- | --- |
| `DSQL_ENDPOINT` | Aurora DSQL 接続先 | DB スタックの CloudFormation Export |
| `DSQL_REGION` | DSQL 接続リージョン | SAM テンプレート固定値 (`ap-northeast-3`) |
| `CORS_ORIGIN` | `Access-Control-Allow-Origin` | Stage ごとの CORS マッピング |

Rust 側の既定 CORS Origin は `https://nishidemasami-github-io-contactform-test.pages.dev` ですが、通常は SAM テンプレートから環境変数で上書きされます。

## エラー動作

| 条件 | ステータス | 備考 |
| --- | --- | --- |
| JWT の `email` / `sub` が欠落、または `sub` が UUID でない | `401` | Lambda 側の追加検査で拒否します。 |
| 未対応メソッド | `405` | Lambda 側で明示的に返します。 |
| DB 接続失敗、INSERT/SELECT 失敗、JSON 解析失敗など | `500` | エラーログ出力後に共通の内部エラーレスポンスを返します。 |

## インフラ依存

| 依存先 | API 側で使うもの | 参照 |
| --- | --- | --- |
| 認証 | Cognito Issuer / User Pool Client ID | [認証](auth.md) |
| DB | `inquiries` テーブル、`selectview` ロール、DSQL endpoint | [データベース](db.md) |
| CI/CD | `cargo check` / `cargo test`、SAM デプロイ、OpenAPI 出力 | [CI/CD](cicd.md) |

## OpenAPI について

`api_cicd.yaml` の `export_openapi` ジョブは、デプロイ後の API Gateway から `/api/openapi.yaml` をエクスポートしてコミットする構成です。ただし、現時点のリポジトリには `api/openapi.yaml` はまだ含まれていません。API 仕様を配布する前提としては CI/CD 側に出口があります。

## 関連ページ

- [認証](auth.md)
- [データベース](db.md)
- [CI/CD](cicd.md)
