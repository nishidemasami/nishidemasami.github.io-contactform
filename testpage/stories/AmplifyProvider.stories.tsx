import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import AmplifyProvider from '../components/AmplifyProvider';

const meta: Meta<typeof AmplifyProvider> = {
  title: 'Components/AmplifyProvider',
  component: AmplifyProvider,
  tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    children: (
      <div className="p-6">
        <h1 className="text-xl font-bold text-gray-800">子コンポーネントのコンテンツ</h1>
        <p className="text-gray-500 mt-2">AmplifyProvider により Cognito 認証が設定されます。</p>
      </div>
    ),
  },
};
