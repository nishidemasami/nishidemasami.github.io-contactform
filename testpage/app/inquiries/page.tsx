'use client';

import { useState, useEffect, useCallback } from 'react';
import { useRouter } from 'next/navigation';
import { getCurrentUser, fetchAuthSession } from 'aws-amplify/auth';
import { InquiriesView, type Inquiry } from './InquiriesView';

/** GET /inquiries APIエンドポイントのレスポンス形式。 */
interface InquiryListResponse {
  email: string;
  count: number;
  inquiries: Inquiry[];
}

/**
 * InquiriesPage コンポーネント。
 *
 * 現在認証されているユーザーのお問い合わせ一覧をバックエンドAPIから取得して表示します。
 * 未認証のユーザーはログインページへリダイレクトされます。
 * 新規お問い合わせを作成するためのリンクも提供します。
 *
 * @returns お問い合わせ一覧ページ、またはデータ取得中のローディングスピナー
 */
export default function InquiriesPage() {
  const router = useRouter();
  const [inquiries, setInquiries] = useState<Inquiry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  const fetchInquiries = useCallback(async () => {
    try {
      const session = await fetchAuthSession();
      const token = session.tokens?.idToken?.toString();
      if (!token) {
        router.push('/');
        return;
      }

      const apiEndpoint = process.env.NEXT_PUBLIC_API_ENDPOINT ?? '';
      const response = await fetch(`${apiEndpoint}/inquiries`, {
        headers: {
          Authorization: `Bearer ${token}`,
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        if (response.status === 401) {
          router.push('/');
          return;
        }
        throw new Error(`APIエラー: ${response.status}`);
      }

      const data: InquiryListResponse = await response.json();
      setInquiries(data.inquiries);
    } catch (err: unknown) {
      if (err instanceof Error) {
        setError(err.message);
      } else {
        setError('データの取得に失敗しました。');
      }
    } finally {
      setLoading(false);
    }
  }, [router]);

  useEffect(() => {
    getCurrentUser()
      .then(() => {
        fetchInquiries();
      })
      .catch(() => {
        router.push('/');
      });
  }, [router, fetchInquiries]);

  return <InquiriesView inquiries={inquiries} loading={loading} error={error} />;
}
