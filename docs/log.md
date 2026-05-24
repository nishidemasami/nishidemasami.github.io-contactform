# 更新ログ

このページは、Wiki の更新内容を時系列で残すためのログです。新しい更新を上に追記します。

## [2026-05-24] ingest | AI自律作業用のプロジェクト管理ファイル追加

- AI エージェントが自律的に開発・修正を進められるように、`docs/TASKS.md` (MoSCoW分析ベースのタスクリスト)、`docs/WBS.md` (アジャイル開発向けWBS)、および `docs/ADR.md` (アーキテクチャ決定レコード) を追加しました。
- 現在のAPIの不足機能 (管理者向けの全体取得・回答APIなど) やフロントエンドのE2Eテスト不足をタスクとして抽出しました。
- `docs/SUMMARY.md` を更新し、プロジェクト管理ファイル群をナビゲーションに追加しました。

## [2026-05-21] lint | Wiki の現行ソース同期

- `docs/README.md` と `docs/SUMMARY.md` を見直し、各トップページへの導線、更新時の着眼点、公開ドキュメントとの関係を現行ワークフローに合わせて整理しました。
- `docs/api.md`、`docs/auth.md`、`docs/db.md`、`docs/cicd.md` を現行ソースに合わせ、Rust からの OpenAPI 生成、API カバレッジ公開、SeaORM 自動生成 PR、Cloudflare Pages 配信フローの説明へ更新しました。
- `docs/FAQ.md` に OpenAPI の正本と公開 README の差異に関する説明を追加し、各ページの相互参照を最新化しました。

## [2026-05-13] lint | 品質担保・GitFlow・クラウド方針の追記

- `docs/README.md` に品質担保（OpenAPI SSoT / Liquibase / Rust の安全性とテスト）、GitFlow（`copilot/** -> develop -> main -> release`）、AWS 方針（Managed / Scalable / Scale to Zero）、llm-wiki 運用方針の要点を追加しました。
- `docs/cicd.md` に品質担保メカニズムと GitHub PR ベースの人間レビューを含む GitFlow の説明を追記しました。
- `docs/FAQ.md` に AWS のマネージド / スケーラブル / Scale to Zero をこのリポジトリ構成でどう実現しているかの説明を追加し、関連ページとの導線を更新しました。

## [2026-05-12] lint | Wiki 全体の同期

- `docs/FAQ.md` に残っていた競合マーカーを除去し、構成図と共通 FAQ を整理しました。
- `docs/api.md`、`docs/auth.md`、`docs/db.md`、`docs/cicd.md` を現行ソースに合わせ、`release` ステージ、`crudrole`、ブランチ別ワークフロー条件、OpenAPI / SeaORM 生成フローの説明を更新しました。
- `docs/README.md` と `docs/SUMMARY.md` の導線を見直し、更新ログへのリンクを追加しました。
