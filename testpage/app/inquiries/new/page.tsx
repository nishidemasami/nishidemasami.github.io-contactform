'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { getCurrentUser, fetchAuthSession } from 'aws-amplify/auth';
import { NewInquiryView } from './NewInquiryView';

/**
 * NewInquiryPage コンポーネント。
 *
 * 認証済みユーザーがバックエンドAPIに新規お問い合わせを送信するためのフォームを提供します。
 * 未認証のユーザーはログインページへリダイレクトされます。
 * 送信成功後はお問い合わせ一覧へ遷移します。
 *
 * @returns 新規お問い合わせフォームページ、または認証状態確認中のローディングスピナー
 */
export default function NewInquiryPage() {
  const router = useRouter();
  const [subject, setSubject] = useState('');
  const [body, setBody] = useState('');
  const [loading, setLoading] = useState(false);
  const [checking, setChecking] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    getCurrentUser()
      .then(() => setChecking(false))
      .catch(() => router.push('/'));
  }, [router]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      const session = await fetchAuthSession();
      const token = session.tokens?.idToken?.toString();
      if (!token) {
        router.push('/');
        return;
      }

      const apiEndpoint = process.env.NEXT_PUBLIC_API_ENDPOINT ?? '';
      const response = await fetch(`${apiEndpoint}/inquiries`, {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${token}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ subject, body }),
      });

      if (!response.ok) {
        if (response.status === 401) {
          router.push('/');
          return;
        }
        throw new Error(`APIエラー: ${response.status}`);
      }

      router.push('/inquiries/index.html');
    } catch (err: unknown) {
      if (err instanceof Error) {
        setError(err.message);
      } else {
        setError('送信に失敗しました。');
      }
    } finally {
      setLoading(false);
    }
  };

  if (checking) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-600" />
      </div>
    );
  }

  return (
    <NewInquiryView
      subject={subject}
      body={body}
      onSubjectChange={setSubject}
      onBodyChange={setBody}
      onSubmit={handleSubmit}
      loading={loading}
      error={error}
    />
  );
}
