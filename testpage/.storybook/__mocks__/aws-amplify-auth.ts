export const getCurrentUser = async () => ({
  username: 'testuser',
  signInDetails: { loginId: 'test@example.com' },
});

export const fetchAuthSession = async () => ({
  tokens: {
    idToken: {
      toString: () => 'mock-id-token',
    },
  },
});

export const signOut = async () => {};

export const signIn = async () => ({ isSignedIn: false, nextStep: { signInStep: 'DONE' } });
export const signUp = async () => ({ isSignUpComplete: false, nextStep: { signUpStep: 'DONE' } });
export const confirmSignIn = async () => ({ isSignedIn: false, nextStep: { signInStep: 'DONE' } });
export const confirmSignUp = async () => ({ isSignUpComplete: true, nextStep: { signUpStep: 'DONE' } });
export const signInWithRedirect = async () => {};
export const resetPassword = async () => ({ isPasswordReset: false, nextStep: { resetPasswordStep: 'DONE' } });
export const confirmResetPassword = async () => {};
export const updatePassword = async () => {};
export const deleteUser = async () => {};
export const fetchUserAttributes = async () => ({});
export const resendSignUpCode = async () => ({ destination: '', deliveryMedium: '', attributeName: '' });
export const confirmUserAttribute = async () => {};
export const sendUserAttributeVerificationCode = async () => ({ destination: '', deliveryMedium: '', attributeName: '' });
export const autoSignIn = async () => ({ isSignedIn: false, nextStep: { signInStep: 'DONE' } });
export const listWebAuthnCredentials = async () => ({ credentials: [] });
export const associateWebAuthnCredential = async () => {};
