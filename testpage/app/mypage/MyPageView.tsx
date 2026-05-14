'use client';

import Link from 'next/link';

/**
 * MyPageView コンポーネント。
 *
 * マイページダッシュボードのUIを表示します。データ取得ロジックを含まず、
 * StorybookなどでUIを独立してテストするために使用します。
 *
 * @param email - ログイン中のユーザーのメールアドレス
 * @param idToken - ログイン中のユーザーのidToken
 * @param signingOut - ログアウト処理中かどうか
 * @param onSignOut - ログアウトボタンのクリックハンドラ
 * @returns マイページダッシュボードUI
 */
export function MyPageView({
  email,
  idToken,
  signingOut,
  onSignOut,
}: {
  email: string;
  idToken: string;
  signingOut: boolean;
  onSignOut: () => void;
}) {
  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
      <header className="bg-white shadow-sm">
        <div className="max-w-5xl mx-auto px-4 py-4 flex items-center justify-between">
          <h1 className="text-xl font-bold text-gray-800">テストページ</h1>
          <button
            onClick={onSignOut}
            disabled={signingOut}
            className="px-4 py-2 bg-red-500 hover:bg-red-600 disabled:bg-red-300 text-white text-sm font-medium rounded-lg transition"
          >
            {signingOut ? 'ログアウト中…' : 'ログアウト'}
          </button>
        </div>
      </header>

      <main className="max-w-5xl mx-auto px-4 py-10">
        <div className="bg-white rounded-2xl shadow-md p-8 mb-8">
          <h2 className="text-2xl font-bold text-gray-800 mb-2">デバッグ情報</h2>
          <p className="text-gray-500">
              <label
                className="block text-sm font-medium text-gray-700 mb-1"
              >
                メールアドレス
              </label>
              <input
                type="text"
                onFocus={function(e: React.FocusEvent<HTMLInputElement>) {
                  e.currentTarget.select(); 
                }}
                readOnly
                value={email}
                className="text-ellipsis w-full px-4 py-3 border text-blue-900 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition"
              />
          </p>
          <p className="text-gray-500">
              <label
                className="block text-sm font-medium text-gray-700 mb-1"
              >
                IDトークン※取り扱い注意⚠️
              </label>
              <input
                type="text"
                onFocus={function(e: React.FocusEvent<HTMLInputElement>) {
                  e.currentTarget.select(); 
                }}
                readOnly
                value={idToken}
                className="text-ellipsis w-full px-4 py-3 border text-blue-900 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition"
              />
          </p>
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
          <Link
            href="/inquiries/index.html"
            className="group bg-white rounded-2xl shadow-md p-6 hover:shadow-lg transition flex flex-col gap-3"
          >
            <div className="w-12 h-12 bg-blue-100 rounded-xl flex items-center justify-center text-blue-600 text-2xl">
              📋
            </div>
            <h3 className="text-lg font-semibold text-gray-800 group-hover:text-blue-600 transition">
              お問い合わせ一覧
            </h3>
            <p className="text-gray-500 text-sm">自分が作成したお問い合わせを確認します。</p>
          </Link>

          <Link
            href="/inquiries/new/index.html"
            className="group bg-white rounded-2xl shadow-md p-6 hover:shadow-lg transition flex flex-col gap-3"
          >
            <div className="w-12 h-12 bg-green-100 rounded-xl flex items-center justify-center text-green-600 text-2xl">
              ✏️
            </div>
            <h3 className="text-lg font-semibold text-gray-800 group-hover:text-green-600 transition">
              新規お問い合わせ
            </h3>
            <p className="text-gray-500 text-sm">新しいお問い合わせを登録します。</p>
          </Link>
        </div>
      </main>
    </div>
  );
}
