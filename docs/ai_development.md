# AI開発

このプロジェクトでは、次のタスクを AI により実施しています。

- プログラムの実装
- プログラムのレビュー
- インフラの構築（IaC の編集）
- テストの作成
- ドキュメントの作成・更新

## 承認フロー内での AI 利用箇所

```plantuml

@startuml
skinparam backgroundColor transparent
skinparam defaultFontname Helvetica
skinparam componentStyle rectangle
skinparam ArrowColor midnightblue
skinparam swimlaneBorderColor White
skinparam swimlaneBorderThickness 0
skinparam ArrowThickness 1

title 図：承認フロー内での AI 利用箇所

|#Honeydew|人間作業|
|#White|CICD| CI/CD
|#Lavenderblush|AI作業|

|人間作業|
:要件・課題の確認;
:Issueの新規作成;

|CICD|
repeat : 以下の操作がされたIssueの検知\n・新規作成\n・差し戻し;
:AIへIssueをアサイン;

|AI作業|
:自律型AIの起動\n例：\n・Jules\n・GitHub Copilot Workspace\n…等; <<continuous>>
partition #white 自律型AI{
  :作業実施\n・IaC作成\n・プログラミング実装\n・自動テスト作成;
  :プルリクエスト作成;
}
|CICD|
:以下の操作がされたIssueの検知\n・プルリクエストの作成;

:ドキュメント生成\n※プログラミング内の\nドキュメンテーションコメント\nから生成する;
note right: 例：\nJava:JavaDoc\nRust:RustDoc\nTypeScript:TypeDoc\nAPI:OpenAPI(Swagger)\n…等
:静的コード解析\n(結果をプルリクエストに添付);
note right: 例：\nJava:FindBugs\nRust:Clippy\nTypeScript:ESLint\n…等
:自動テスト実施\n(結果をプルリクエストに添付);
note right: 例：\nJenkins\nGitHub Actions\n…等
:AIへコードレビューをアサイン;
|AI作業|
:エージェント型AIの起動\n例：\n・Gemini Code Assist\n・GitHub Copilot Agent\n…等; <<continuous>>

partition #white エージェント型AI{
  :コードレビュー(AI);
}
|人間作業|
repeatwhile (\nコードレビュー(人間)\n・ドキュメントの確認\n・静的コード解析結果の確認\n・自動テスト結果の確認\n・コードレビュー(AI)結果の確認\n・手動テスト実施(必要に応じて)\n) is (NG：\n差し戻し)
-> OK：\nプルリクエストの承認;
:プルリクエストのマージ;
|CICD|
:以下の操作がされたIssueの検知\n・ブランチ(develop等)へのプッシュ;
:環境(develop等)へのデプロイ;
note right: 例：\nAWS：CloudFormation\nAzure：Resource Manager\nGCP：TerraForm on GCP\n…等

|人間作業|
:デプロイ結果の確認;
:動作確認・テスト実施(必要に応じて);
detach
@enduml

```
