# FAQ

## ブラウザから API Gateway、Lambda、Aurora DSQL への流れは？

```plantuml
@startuml
left to right direction
skinparam backgroundColor transparent
skinparam defaultFontname Meiryo
skinparam componentStyle rectangle

actor 利用者 as User

package "公開フロントエンド" {
  [Cloudflare Pages]
  [testpage\n(Next.js static export)]
}

package "認証基盤" {
  [Cognito User Pool]
}

package "バックエンド" {
  [HTTP API Gateway]
  [Rust Lambda]
}

package "DB" {
  [Aurora DSQL]
}

User --> "Cloudflare Pages" : HTTPS
"Cloudflare Pages" --> "testpage\n(Next.js static export)" : 静的配信
User --> "Cognito User Pool" : サインイン
"testpage\n(Next.js static export)" --> "Cognito User Pool" : Amplify Auth
"testpage\n(Next.js static export)" --> "HTTP API Gateway" : Bearer JWT 付き GET/POST /inquiries
"HTTP API Gateway" --> "Cognito User Pool" : JWT 検証
"HTTP API Gateway" --> "Rust Lambda" : ルーティング
"Rust Lambda" --> "Aurora DSQL" : crudrole で SELECT / INSERT
@enduml
```

補足:

- JWT の Issuer / Audience は [認証](auth.md) の CloudFormation Export を [API](api.md) が `Fn::ImportValue` で参照します。
- `testpage/` は `NEXT_PUBLIC_USER_POOL_ID`、`NEXT_PUBLIC_USER_POOL_CLIENT_ID`、`NEXT_PUBLIC_API_ENDPOINT` を [CI/CD](cicd.md) で注入して静的ビルドされます。
- DB への接続先は [データベース](db.md) の `DSQLClusterEndpoint` Export を使います。

## スタック名の接頭辞が `snngicf` なのはなぜですか？

`StackNamePrefix` の既定値が `snngicf` なのは、CloudFormation / AWS リソース名が長くなりすぎるのを避けるためです。元のリポジトリ名 `nishidemasami-github-io-contactform` を短縮した値で、[認証](auth.md)・[データベース](db.md)・[API](api.md) の各 SAM テンプレートで共通に使われています。

## Wiki を更新しても公開ドキュメントが自動再配信されないのはなぜですか？

公開ドキュメントを Cloudflare Pages へ配信するのは [CI/CD](cicd.md) の `document_cicd.yaml` ですが、このワークフローは `docs/**` を監視していません。現状の自動トリガーは `.github/workflows/document_cicd.yaml` と `testpage/**` の変更、または手動実行だけです。

そのため、`docs/` だけを更新した場合は Wiki の内容自体は Git に残っても、公開ページへ反映するには `document_cicd.yaml` を手動実行する必要があります。
