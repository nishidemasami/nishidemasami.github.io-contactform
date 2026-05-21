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
skinparam defaultFontname Meiryo
skinparam componentStyle rectangle

title 開発フローと AI 利用箇所

start
:要件・課題の確認;
:AI によるプログラム実装;
:AI による IaC 編集;
:AI によるテスト作成;
:AI によるドキュメント作成・更新;
:AI によるレビュー;
:人間による確認・承認;
if (承認されたか？) then (Yes)
  :マージ・デプロイ;
  stop
else (No)
  :差し戻し;
  :AI で修正;
  -> :人間による確認・承認;
endif
@enduml
```
