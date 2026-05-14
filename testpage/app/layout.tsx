import type { Metadata } from 'next';
import './globals.css';
import '@aws-amplify/ui-react/styles.css';
import AmplifyProvider from '../components/AmplifyProvider';

export const metadata: Metadata = {
  title: 'testpage',
  description: 'testpage with AWS Amplify and Cognito',
};

/**
 * RootLayout コンポーネント。
 *
 * アプリケーションのルートHTML構造を提供し、グローバルスタイルを適用するとともに、
 * すべてのページをAmplifyProviderでラップしてAWS Cognito認証を有効にします。
 *
 * @param props - レイアウト内にレンダリングするReactの子要素
 * @returns AmplifyProviderでラップされた子要素を含むルートHTMLドキュメント
 */
export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="ja">
      <body className="bg-gray-50 min-h-screen">
        <AmplifyProvider>{children}</AmplifyProvider>
      </body>
    </html>
  );
}
