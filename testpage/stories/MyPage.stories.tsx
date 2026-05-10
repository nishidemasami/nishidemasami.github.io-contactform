import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { MyPageView } from '../app/mypage/MyPageView';

const meta: Meta<typeof MyPageView> = {
  title: 'Pages/MyPage',
  component: MyPageView,
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
    email: 'test@example.com',
    signingOut: false,
    onSignOut: () => {},
  },
};

export const SigningOut: Story = {
  args: {
    email: 'test@example.com',
    signingOut: true,
    onSignOut: () => {},
  },
};
