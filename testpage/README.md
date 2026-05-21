# テストページ

## 開発の開始方法

### pnpmのインストール

```
curl -fsSL https://get.pnpm.io/install.sh | sh -
```

### 環境変数の設定
- NEXT_PUBLIC_USER_POOL_ID=
- NEXT_PUBLIC_USER_POOL_CLIENT_ID= 
- NEXT_PUBLIC_API_ENDPOINT=

### 各コマンド
- `pnpm exec tsc --noEmit` ： 型チェック
- `pnpm lint` ： リントチェック
- `pnpm dev` ： 開発モード起動
- `pnpm build` ： Next.jsのビルド
- `pnpm build-storybook` ： StoryBookのビルド

### 各ファイルの説明[WIP]
- app
  - globals.css
  - inquiries
    - InquiriesView.tsx
    - new
      - NewInquiryView.tsx
      - page.tsx
    - page.tsx
  - layout.tsx
  - mypage
    - MyPageView.tsx
    - page.tsx
  - page.tsx
- components
  - AmplifyProvider.tsx
- next.config.js
- pnpm-lock.yaml
- pnpm-workspace.yaml
- package.json
- postcss.config.js
- stories
  - AmplifyProvider.stories.tsx
  - InquiriesPage.stories.tsx
  - LoginPage.stories.tsx
  - MyPage.stories.tsx
  - NewInquiryPage.stories.tsx
- tailwind.config.js
- template.yaml
- tsconfig.json

TODO:追記
