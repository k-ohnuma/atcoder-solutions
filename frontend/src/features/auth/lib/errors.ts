import { FirebaseError } from "firebase/app";

export function toFirebaseAuthErrorMessage(error: unknown): string {
  if (!(error instanceof FirebaseError)) {
    return "認証処理でエラーが発生しました。時間をおいて再度お試しください。";
  }

  switch (error.code) {
    case "auth/email-already-in-use":
      return "このメールアドレスは既に登録されています。";
    case "auth/invalid-email":
      return "メールアドレスの形式が正しくありません。";
    case "auth/weak-password":
      return "パスワードが弱すぎます。";
    case "auth/network-request-failed":
      return "ネットワークエラーが発生しました。接続を確認してください。";
    case "auth/too-many-requests":
      return "試行回数が多すぎます。しばらく待ってから再度お試しください。";
    case "auth/invalid-credential":
      return "メールアドレスまたはパスワードが間違っています。";
    case "auth/requires-recent-login":
      return "安全のため、再ログイン後にもう一度お試しください。";
    default:
      return "認証処理でエラーが発生しました。入力内容を確認して再度お試しください。";
  }
}

export function toCreateUserApiErrorMessage(status: number, error: unknown): string {
  if (status === 401) {
    return "認証に失敗しました。再度ログインしてからお試しください。";
  }
  if (status === 409) {
    return "このユーザー名は既に使われています。別のユーザー名を入力してください。";
  }
  if (status >= 500) {
    return "ユーザー登録に失敗しました。時間をおいて再度お試しください。";
  }
  if (typeof error === "string" && error.length > 0) {
    return error;
  }
  return "ユーザー登録に失敗しました。入力内容を確認してください。";
}
