'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { getCurrentUser, signOut, fetchAuthSession } from 'aws-amplify/auth';
import { MyPageView } from './MyPageView';

/**
 * MyPage コンポーネント。
 *
 * 認証済みユーザーに表示されるダッシュボードページです。ログイン中のユーザーの
 * メールアドレスを表示し、お問い合わせ一覧や新規お問い合わせフォームへのナビゲーション
 * リンクを提供します。サインアウトボタンも含まれています。
 *
 * @returns マイページダッシュボード、または認証状態確認中のローディングスピナー
 */
export default function MyPage() {
  const router = useRouter();
  const [email, setEmail] = useState('');
  const [idToken, setIdToken] = useState('');
  const [loading, setLoading] = useState(true);
  const [signingOut, setSigningOut] = useState(false);

  useEffect(() => {
    getCurrentUser()
      .then(async (user) => {
        setEmail(user.signInDetails?.loginId ?? user.username);
        //idTokenを取得
        const session = await fetchAuthSession();
        const token = session?.tokens?.idToken?.toString();
        setIdToken(token);
        setLoading(false);
      })
      .catch(() => {
        router.push('/');
      });
  }, [router]);

  const handleSignOut = async () => {
    setSigningOut(true);
    try {
      await signOut();
      router.push('/');
    } catch {
      setSigningOut(false);
    }
  };

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-600" />
      </div>
    );
  }

  return <MyPageView email={email} idToken={idToken} signingOut={signingOut} onSignOut={handleSignOut} />;
}
