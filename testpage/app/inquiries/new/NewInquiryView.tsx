'use client';

import Link from 'next/link';

/**
 * NewInquiryView コンポーネント。
 *
 * 新規お問い合わせフォームのUIを表示します。データ送信ロジックを含まず、
 * StorybookなどでUIを独立してテストするために使用します。
 *
 * @param subject - 件名の現在値
 * @param body - 内容の現在値
 * @param onSubjectChange - 件名変更ハンドラ
 * @param onBodyChange - 内容変更ハンドラ
 * @param onSubmit - フォーム送信ハンドラ
 * @param loading - 送信処理中かどうか
 * @param error - エラーメッセージ（空文字列はエラーなし）
 * @returns 新規お問い合わせフォームUI
 */
export function NewInquiryView({
  subject,
  body,
  onSubjectChange,
  onBodyChange,
  onSubmit,
  loading,
  error,
}: {
  subject: string;
  body: string;
  onSubjectChange: (value: string) => void;
  onBodyChange: (value: string) => void;
  onSubmit: (e: React.FormEvent) => void;
  loading: boolean;
  error: string;
}) {
  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
      <header className="bg-white shadow-sm">
        <div className="max-w-5xl mx-auto px-4 py-4">
          <nav className="flex items-center gap-2 text-sm text-gray-500">
            <Link href="/mypage/index.html" className="hover:text-blue-600 transition">
              マイページ
            </Link>
            <span>/</span>
            <Link href="/inquiries/index.html" className="hover:text-blue-600 transition">
              お問い合わせ一覧
            </Link>
            <span>/</span>
            <span className="text-gray-700 font-medium">新規登録</span>
          </nav>
        </div>
      </header>

      <main className="max-w-2xl mx-auto px-4 py-10">
        <div className="bg-white rounded-2xl shadow-md p-8">
          <h2 className="text-2xl font-bold text-gray-800 mb-6">新規お問い合わせ</h2>

          {error && (
            <div className="mb-5 p-3 bg-red-50 border border-red-200 rounded-lg text-red-700 text-sm">
              {error}
            </div>
          )}

          <form onSubmit={onSubmit} className="space-y-5">
            <div>
              <label
                htmlFor="subject"
                className="block text-sm font-medium text-gray-700 mb-1"
              >
                件名 <span className="text-red-500">*</span>
              </label>
              <input
                id="subject"
                type="text"
                required
                value={subject}
                onChange={(e) => onSubjectChange(e.target.value)}
                className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition"
                placeholder="お問い合わせの件名を入力してください"
              />
            </div>

            <div>
              <label
                htmlFor="body"
                className="block text-sm font-medium text-gray-700 mb-1"
              >
                内容 <span className="text-red-500">*</span>
              </label>
              <textarea
                id="body"
                required
                rows={6}
                value={body}
                onChange={(e) => onBodyChange(e.target.value)}
                className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition resize-y"
                placeholder="お問い合わせの詳細を入力してください"
              />
            </div>

            <div className="flex gap-3 pt-2">
              <Link
                href="/inquiries/index.html"
                className="flex-1 py-3 px-4 bg-gray-100 hover:bg-gray-200 text-gray-700 font-semibold rounded-lg transition text-center"
              >
                キャンセル
              </Link>
              <button
                type="submit"
                disabled={loading}
                className="flex-1 py-3 px-4 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-300 text-white font-semibold rounded-lg transition flex items-center justify-center"
              >
                {loading ? (
                  <span className="animate-spin rounded-full h-5 w-5 border-t-2 border-b-2 border-white" />
                ) : (
                  '送信する'
                )}
              </button>
            </div>
          </form>
        </div>
      </main>
    </div>
  );
}
