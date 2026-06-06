# Work Breakdown Structure (WBS)

アジャイル開発向けの WBS です。`docs/TASKS.md` に記載されたバックログをエピック (Epic)、ストーリー (Story)、タスク (Task) に分割し、AI がイテレーション (Sprint) で取り組めるような実行可能なサイズに落とし込んでいます。

## Epic 1: 管理者向け問い合わせ対応機能の提供
目的: 管理者が全ユーザーの問い合わせを閲覧し、回答を書き込めるようにする。

### Story 1.1: 管理者権限の導入
- [ ] Task 1.1.1: Cognito User Pool に `Admin` グループを追加する設定を `infrastructure/auth/template.yaml` に追記する。
- [ ] Task 1.1.2: API Gateway の Authorizer 設定で、特定の API へのアクセスを `Admin` グループのみに制限する方法を検証・実装する。

### Story 1.2: 全ての問い合わせを取得するAPIの実装
- [ ] Task 1.2.1: `api/lambda/src/handlers.rs` に `GET /admin/inquiries` を処理するハンドラ (`handle_get_all_inquiries`) を追加する。
- [ ] Task 1.2.2: `GET /admin/inquiries` のための OpenAPI スキーマを `generate-openapi.rs` に追加する。
- [ ] Task 1.2.3: `api/template.yaml` に `GET /admin/inquiries` のエンドポイント定義を追加する。
- [ ] Task 1.2.4: ローカル PostgreSQL と Liquibase を使った統合テストを追加し、機能とカバレッジを確認する。

### Story 1.3: 問い合わせに回答するAPIの実装
- [ ] Task 1.3.1: `api/lambda/src/handlers.rs` に `PUT /admin/inquiries/{id}/reply` を処理するハンドラ (`handle_put_inquiry_reply`) を追加する。
- [ ] Task 1.3.2: 該当ハンドラ内で、DB の `reply`, `respondent`, `reply_at` カラムを更新するロジックを実装する。
- [ ] Task 1.3.3: OpenAPI スキーマと `template.yaml` の更新を行う。
- [ ] Task 1.3.4: 該当機能のテストを追加する。

### Story 1.4: フロントエンドへの管理者画面の追加
- [ ] Task 1.4.1: Next.js アプリ (`testpage/`) に管理者用ページ (`/admin`) を作成する。
- [ ] Task 1.4.2: Cognito ログイン情報から `Admin` グループに属しているか判定し、管理者用画面へのアクセス制御を行う。
- [ ] Task 1.4.3: 全問い合わせ一覧を表示する UI コンポーネントを作成する。
- [ ] Task 1.4.4: 選択した問い合わせに対して回答を入力し、`PUT` リクエストを送信するフォームを作成する。


## Epic 2: フロントエンドの品質保証強化
目的: Next.js のフロントエンドコードに対する E2E テストを導入し、AI による変更時のリグレッションを防ぐ。

### Story 2.1: E2E テスト基盤の導入
- [ ] Task 2.1.1: `testpage` ディレクトリに Playwright (または Cypress) をインストール・設定する。
- [ ] Task 2.1.2: `package.json` に E2E テスト実行用のスクリプト (`test:e2e` など) を追加する。
- [ ] Task 2.1.3: CI/CD パイプライン (`document_cicd.yaml` など) で E2E テストを実行するように GitHub Actions を更新する。

### Story 2.2: 主要なユースケースのテスト作成
- [ ] Task 2.2.1: ログイン〜ログアウトのフローを確認する E2E テストを作成する。
- [ ] Task 2.2.2: 新規問い合わせの作成フォーム入力〜送信のフローを確認する E2E テストを作成する。
- [ ] Task 2.2.3: 問い合わせ一覧画面の表示を確認する E2E テストを作成する。


## Epic 3: ユーザーの問い合わせ取り消し機能
目的: ユーザーが誤って送信した問い合わせを自ら削除できるようにする。

### Story 3.1: 問い合わせ削除 API の実装
- [ ] Task 3.1.1: `api/lambda/src/handlers.rs` に `DELETE /inquiries/{id}` ハンドラを追加する。
- [ ] Task 3.1.2: 所有権チェック（JWTの `cognito_sub` と DBの `cognito_sub` が一致するか）を実装する。
- [ ] Task 3.1.3: OpenAPI スキーマ、`template.yaml` の更新、テストの追加を行う。

### Story 3.2: フロントエンドからの削除呼び出し
- [ ] Task 3.2.1: Next.js の問い合わせ一覧画面に「削除」ボタンを追加する。
- [ ] Task 3.2.2: 削除確認のモーダル（またはダイアログ）を実装し、誤操作を防ぐ。
- [ ] Task 3.2.3: 削除成功時に一覧をリロードまたは状態更新するロジックを実装する。
