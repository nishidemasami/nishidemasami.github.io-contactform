# 認証

## 概要

認証基盤は `infrastructure/auth/template.yaml` の AWS SAM テンプレートで管理され、Amazon Cognito の **User Pool** と **User Pool Client** を `develop` / `main` 環境ごとにデプロイします。

- テンプレート: [`../infrastructure/auth/template.yaml`](../infrastructure/auth/template.yaml)
- 補足 README: [`../infrastructure/auth/README.md`](../infrastructure/auth/README.md)
- デプロイ: [CI/CD](cicd.md)

## デプロイ対象

| リソース | 役割 | 命名規則 |
| --- | --- | --- |
| `CognitoUserPool` | ユーザー管理本体 | `${StackNamePrefix}-user-pool-${Stage}` |
| `CognitoUserPoolClient` | API / フロントエンドが使うアプリクライアント | `${StackNamePrefix}-app-client-${Stage}` |

`StackNamePrefix` の既定値は `snngicf`です。これは、各リソースの名前が長すぎる場合にエラーとなる場合があるためです。
`Stage` の規定値は `develop` または `main` です。

## User Pool 設定

- メールアドレスを自動検証します。
- ユーザー名属性としてメールアドレスを使います。
- Hosted UI や外部 IdP の設定は、現在のテンプレートには含まれていません。

| 項目 | 値 |
| --- | --- |
| 最小長 | 8 |
| 英大文字 | 必須 |
| 英小文字 | 必須 |
| 数字 | 必須 |
| 記号 | 不要 |

## User Pool Client 設定

- `ALLOW_USER_SRP_AUTH`
- `ALLOW_REFRESH_TOKEN_AUTH`
- `GenerateSecret: false`
- `PreventUserExistenceErrors: ENABLED`

現状のテンプレートでは、API Gateway JWT Authorizer から利用する前提のシンプルな App Client 構成になっています。

## CloudFormation Outputs

| Output | 用途 | API からの利用 |
| --- | --- | --- |
| `CognitoUserPoolId` | User Pool ID の参照 | フロントエンド設定や運用参照用 |
| `CognitoUserPoolClientId` | App Client ID の参照 | API Gateway の JWT Audience |
| `CognitoIssuer` | JWT Issuer URL | API Gateway の JWT Issuer |

これらは `${StackNamePrefix}-auth-${Stage}-...` 形式で Export され、[API](api.md) の SAM テンプレートから `Fn::ImportValue` で参照されます。

## ブランチとステージ

`cognito_cicd.yaml` は `main` / `develop` ブランチの push・pull request・手動実行に対応しています。デプロイ時は `github.ref_name` をそのまま `Stage` に渡すため、ブランチ運用と Cognito 環境名が一致します。

## 関連ページ

- [API](api.md)
- [データベース](db.md)
- [CI/CD](cicd.md)
