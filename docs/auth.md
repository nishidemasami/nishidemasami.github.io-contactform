# 認証

## 概要

認証基盤は AWS SAM テンプレート `infrastructure/auth/template.yaml` で管理され、Amazon Cognito の **User Pool** と **User Pool Client** を環境ごとにデプロイします。

- ソース: [`../infrastructure/auth/template.yaml`](../infrastructure/auth/template.yaml)
- 補足 README: [`../infrastructure/auth/README.md`](../infrastructure/auth/README.md)
- デプロイ経路: [CI/CD](cicd.md)

## デプロイ対象

| リソース | 役割 | 命名規則 |
| --- | --- | --- |
| `CognitoUserPool` | ユーザー管理本体 | `${StackNamePrefix}-user-pool-${Stage}` |
| `CognitoUserPoolClient` | アプリケーションクライアント | `${StackNamePrefix}-app-client-${Stage}` |

## 設定内容

### User Pool

- メールアドレスを自動検証します。
- ユーザー名属性としてメールアドレスを使用します。
- パスワードポリシーは次の通りです。

| 項目 | 値 |
| --- | --- |
| 最小長 | 8 |
| 英大文字 | 必須 |
| 英小文字 | 必須 |
| 数字 | 必須 |
| 記号 | 不要 |

### User Pool Client

- `ALLOW_USER_SRP_AUTH`
- `ALLOW_REFRESH_TOKEN_AUTH`
- クライアントシークレットは生成しません (`GenerateSecret: false`)。
- 存在しないユーザーへの応答差分を抑制します (`PreventUserExistenceErrors: ENABLED`)。

## CloudFormation Outputs

| Output | 用途 |
| --- | --- |
| `CognitoUserPoolId` | User Pool ID の参照 |
| `CognitoUserPoolClientId` | App Client ID の参照 |
| `CognitoIssuer` | JWT 検証に使う Issuer URL |

`CognitoIssuer` は API 側で JWT を検証する際の基準値になります。API 実装が追加されたら、[API](api.md) でこの Output の利用箇所を明記します。

## ステージ

`Stage` パラメータは `develop` と `main` の 2 値です。GitHub Actions ではブランチ名をそのまま `Stage` に渡すため、ブランチ運用とデプロイ先が対応しています。

## 関連ページ

- [API](api.md)
- [データベース](db.md)
- [CI/CD](cicd.md)
