import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { NewInquiryView } from '../app/inquiries/new/NewInquiryView';

const meta: Meta<typeof NewInquiryView> = {
  title: 'Pages/NewInquiryPage',
  component: NewInquiryView,
  tags: ['autodocs'],
  parameters: {
    nextjs: {
      appDirectory: true,
    },
  },
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    subject: '',
    body: '',
    onSubjectChange: () => {},
    onBodyChange: () => {},
    onSubmit: (e) => e.preventDefault(),
    loading: false,
    error: '',
  },
};

export const WithData: Story = {
  args: {
    subject: 'テストの件名',
    body: 'テストのお問い合わせ内容です。',
    onSubjectChange: () => {},
    onBodyChange: () => {},
    onSubmit: (e) => e.preventDefault(),
    loading: false,
    error: '',
  },
};

export const Submitting: Story = {
  args: {
    subject: 'テストの件名',
    body: 'テストのお問い合わせ内容です。',
    onSubjectChange: () => {},
    onBodyChange: () => {},
    onSubmit: (e) => e.preventDefault(),
    loading: true,
    error: '',
  },
};

export const WithError: Story = {
  args: {
    subject: 'テストの件名',
    body: 'テストのお問い合わせ内容です。',
    onSubjectChange: () => {},
    onBodyChange: () => {},
    onSubmit: (e) => e.preventDefault(),
    loading: false,
    error: '送信に失敗しました。',
  },
};
