# FAQ

## ブラウザからAPI Gateway、Lambda、DSQLへの流れは？

```plantuml
left to right direction
skinparam backgroundColor transparent 
skinparam defaultFontname Meiryo
skinparam componentStyle rectangle

actor ブラウザ as User

package "フロントエンド" {
  [CloudFront]
  [S3]
}

package "認証基盤" {
  [Cognito]
}

package "バックエンド" {
  [API Gateway]
  [Lambda]

}

package "DB" {
  [DynamoDB]
  [Aurora DSQL]
}

User --> CloudFront : HTTPS
User --> Cognito : 認証
CloudFront --> S3 : 静的コンテンツ

User --> "API Gateway" : APIリクエスト
"API Gateway" --> Lambda : 実行

Lambda --> DB : データの登録・更新
"API Gateway" --> Cognito : 認証検証
```

## スタック名の接頭辞は、なぜ `snngicf` というわかりにくい名前なのですか？
これは、各リソースの名前が長すぎる場合にエラーとなる場合があるためです。  
なお、`snngicf` という値は、当初の `nishidemasami-github-io-contactform` を短縮したものに由来しています。