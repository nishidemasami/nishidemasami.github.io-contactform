'use client';

import { Amplify } from 'aws-amplify';
import { translations } from '@aws-amplify/ui-react';
import { I18n } from 'aws-amplify/utils';

Amplify.configure({
  Auth: {
    Cognito: {
      userPoolId: process.env.NEXT_PUBLIC_USER_POOL_ID ?? '',
      userPoolClientId: process.env.NEXT_PUBLIC_USER_POOL_CLIENT_ID ?? '',
    },
  },
});

I18n.putVocabularies(translations);
I18n.setLanguage('ja');

/**
 * AmplifyProvider コンポーネント。
 *
 * Cognito認証設定でAWS Amplifyを構成し、アプリケーションツリーをラップすることで
 * すべての子コンポーネントからAmplify Auth APIを利用できるようにします。
 *
 * @param props - プロバイダー内にレンダリングするReactの子要素
 * @returns 追加のDOM要素なしにレンダリングされた子要素
 */
export default function AmplifyProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  return <>{children}</>;
}
