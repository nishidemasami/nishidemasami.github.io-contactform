import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { InquiriesView } from '../app/inquiries/InquiriesView';

const meta: Meta<typeof InquiriesView> = {
  title: 'Pages/InquiriesPage',
  component: InquiriesView,
  tags: ['autodocs'],
  parameters: {
    nextjs: {
      appDirectory: true,
    },
  },
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Loading: Story = {
  args: {
    inquiries: [],
    loading: true,
    error: '',
  },
};

export const Default: Story = {
  args: {
    inquiries: [],
    loading: false,
    error: '',
  },
};

export const WithData: Story = {
  args: {
    inquiries: [
      {
        id: '1',
        email: 'test@example.com',
        subject: 'テストお問い合わせ 1',
        body: 'これはテスト用のお問い合わせ本文です。',
        created_at: '2024-01-15T10:30:00Z',
      },
      {
        id: '2',
        email: 'test@example.com',
        subject: 'テストお問い合わせ 2',
        body: '別のテスト用のお問い合わせ本文です。詳細な内容を入力します。',
        created_at: '2024-01-16T14:20:00Z',
      },
    ],
    loading: false,
    error: '',
  },
};

export const WithError: Story = {
  args: {
    inquiries: [],
    loading: false,
    error: 'データの取得に失敗しました。',
  },
};
