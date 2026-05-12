'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { Authenticator, translations } from '@aws-amplify/ui-react';
import { I18n } from 'aws-amplify/utils';

I18n.putVocabularies(translations);
I18n.setLanguage('ja');

/**
 * AuthenticatedRedirect コンポーネント。
 *
 * ログイン成功後、認証済みユーザーをマイページにリダイレクトします。
 * ユーザーセッションが有効な場合、Authenticatorコンポーネント内でレンダリングされます。
 *
 * @returns null — クライアントサイドのナビゲーション副作用のみを実行します
 */
function AuthenticatedRedirect() {
  const router = useRouter();
  useEffect(() => {
    router.push('/mypage/index.html');
  }, [router]);
  return null;
}

/**
 * LoginPage コンポーネント。
 *
 * AWS Amplify Authenticator UIを使用してアプリケーションのログイン画面をレンダリングします。
 * 認証済みユーザーは自動的にマイページへリダイレクトされます。
 *
 * @returns Amplify Authenticatorウィジェットを含むログインページ
 */
export default function LoginPage() {
  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 to-indigo-100">
      <Authenticator loginMechanisms={['email']}>
        {({ user }) => (user ? <AuthenticatedRedirect /> : <></>)}
      </Authenticator>
    </div>
  );
}
