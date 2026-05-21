# 認証

## 概要

認証基盤は `infrastructure/auth/template.yaml` の AWS SAM テンプレートで管理され、Amazon Cognito の **User Pool** と **User Pool Client** を `develop` / `main` / `release` 環境向けに定義します。

- テンプレート: [`../infrastructure/auth/template.yaml`]((https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/infrastructure/auth/template.yaml)
- 補足 README: [`../infrastructure/auth/README.md`]((https://github.com/nishidemasami/nishidemasami.github.io-contactform/blob/main/infrastructure/auth/README.md)
- 関連デプロイ: [CI/CD](cicd.md)

## デプロイ対象

| リソース | 役割 | 命名規則 |
| --- | --- | --- |
| `CognitoUserPool` | ユーザー管理本体 | `${StackNamePrefix}-user-pool-${Stage}` |
| `CognitoUserPoolClient` | API / testpage が使うアプリクライアント | `${StackNamePrefix}-app-client-${Stage}` |

`StackNamePrefix` の既定値は `snngicf`、`Stage` の既定値は `develop` です。テンプレートが許可する Stage は `develop` / `main` / `release` です。

## User Pool 設定

- メールアドレスを自動検証します。
- ユーザー名属性としてメールアドレスを使います。
- Hosted UI、独自ドメイン、外部 IdP の設定は含まれていません。

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

現在のテンプレートは、[API](api.md) の JWT Authorizer と `testpage/` の Amplify 設定で使う、シンプルな App Client を用意する構成です。

## CloudFormation Outputs

| Output | Export 名 | 主な利用先 |
| --- | --- | --- |
| `CognitoUserPoolId` | `${StackNamePrefix}-auth-${Stage}-CognitoUserPoolId` | `testpage` ビルド時の `NEXT_PUBLIC_USER_POOL_ID` |
| `CognitoUserPoolClientId` | `${StackNamePrefix}-auth-${Stage}-CognitoUserPoolClientId` | API Gateway JWT Audience、`NEXT_PUBLIC_USER_POOL_CLIENT_ID` |
| `CognitoIssuer` | `${StackNamePrefix}-auth-${Stage}-CognitoIssuer` | API Gateway JWT Issuer |

`document_cicd.yaml` はこれらの Export を `aws cloudformation list-exports` で取得し、`testpage` の静的ビルドと Swagger UI 用の認証設定に注入します。

## ブランチとステージ

`cognito_cicd.yaml` は `develop` / `release` への push、`main` / `develop` / `release` 向け pull request、`workflow_dispatch` に対応しています。`sam deploy` が走るのは pull request 以外なので、`main` 環境は主に手動実行で更新する想定です。

| 実行契機 | 検証 | デプロイ |
| --- | --- | --- |
| `develop` / `release` への push | あり | あり |
| `main` / `develop` / `release` 向け pull request | あり | なし |
| `workflow_dispatch` | あり | あり |

デプロイ時は `github.ref_name` をそのまま `Stage` に渡します。

## API / フロントエンドとの接続

- [API](api.md) は `Fn::ImportValue` で `CognitoIssuer` と `CognitoUserPoolClientId` を読み込み、HTTP API の既定 Authorizer に設定します。
- `testpage/components/AmplifyProvider.tsx` は User Pool ID と Client ID を受け取り、AWS Amplify の Cognito 設定に使います。
- 認証済み画面では `fetchAuthSession()` から取得した ID トークンを `Authorization: Bearer ...` として API に渡します。

## 関連ページ

- [FAQ](FAQ.md)
- [API](api.md)
- [CI/CD](cicd.md)
