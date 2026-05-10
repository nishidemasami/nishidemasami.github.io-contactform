'use client';

import Link from 'next/link';

/** APIから返される単一のお問い合わせを表します。 */
export interface Inquiry {
  id: string;
  email: string;
  subject: string;
  body: string;
  created_at: string;
}

/**
 * InquiriesView コンポーネント。
 *
 * お問い合わせ一覧のUIを表示します。データ取得ロジックを含まず、
 * StorybookなどでUIを独立してテストするために使用します。
 *
 * @param inquiries - 表示するお問い合わせの配列
 * @param loading - データ取得中かどうか
 * @param error - エラーメッセージ（空文字列はエラーなし）
 * @returns お問い合わせ一覧UI、またはデータ取得中のローディングスピナー
 */
export function InquiriesView({
  inquiries,
  loading,
  error,
}: {
  inquiries: Inquiry[];
  loading: boolean;
  error: string;
}) {
  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-600" />
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
      <header className="bg-white shadow-sm">
        <div className="max-w-5xl mx-auto px-4 py-4 flex items-center justify-between">
          <nav className="flex items-center gap-2 text-sm text-gray-500">
            <Link href="/mypage/index.html" className="hover:text-blue-600 transition">
              マイページ
            </Link>
            <span>/</span>
            <span className="text-gray-700 font-medium">お問い合わせ一覧</span>
          </nav>
          <Link
            href="/inquiries/new/index.html"
            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium rounded-lg transition"
          >
            + 新規作成
          </Link>
        </div>
      </header>

      <main className="max-w-5xl mx-auto px-4 py-10">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-2xl font-bold text-gray-800">お問い合わせ一覧</h2>
          <span className="text-sm text-gray-500">{inquiries.length} 件</span>
        </div>

        {error && (
          <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg text-red-700">
            {error}
          </div>
        )}

        {inquiries.length === 0 && !error ? (
          <div className="bg-white rounded-2xl shadow-md p-12 text-center">
            <p className="text-gray-400 text-lg mb-4">お問い合わせがありません。</p>
            <Link
              href="/inquiries/new/index.html"
              className="inline-block px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition"
            >
              最初のお問い合わせを作成する
            </Link>
          </div>
        ) : (
          <div className="space-y-4">
            {inquiries.map((inquiry) => (
              <div
                key={inquiry.id}
                className="bg-white rounded-2xl shadow-md p-6 hover:shadow-lg transition"
              >
                <div className="flex items-start justify-between gap-4">
                  <div className="flex-1 min-w-0">
                    <h3 className="text-lg font-semibold text-gray-800 truncate">
                      {inquiry.subject}
                    </h3>
                    <p className="mt-1 text-gray-500 text-sm line-clamp-2">{inquiry.body}</p>
                  </div>
                  <time className="text-xs text-gray-400 whitespace-nowrap mt-1">
                    {new Date(inquiry.created_at).toLocaleString('ja-JP')}
                  </time>
                </div>
              </div>
            ))}
          </div>
        )}
      </main>
    </div>
  );
}
