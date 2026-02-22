import { FirebaseError } from "firebase/app";
import {
  createUserWithEmailAndPassword,
  deleteUser,
  signInWithEmailAndPassword,
  signOut,
  type UserCredential,
} from "firebase/auth";
import { getFirebaseAuth } from "@/shared/firebase/client";
import { SignInSchema, SignUpSchema } from "../model/schema";

function toFirebaseAuthErrorMessage(error: unknown): string {
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

function toCreateUserApiErrorMessage(status: number, error: unknown): string {
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

export const onSubmitSignUp = async (values: SignUpSchema) => {
  const auth = getFirebaseAuth();
  const email = values.email;
  const password = values.password;
  let cred: UserCredential;
  try {
    cred = await createUserWithEmailAndPassword(auth, email, password);
  } catch (e) {
    throw new Error(toFirebaseAuthErrorMessage(e));
  }

  // firebaseが成功して、DBへの格納がこけたらfirebaseもロールバックしなきゃ
  try {
    const idToken = await cred.user.getIdToken();
    const res = await fetch("/api/users", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${idToken}`,
      },
      body: JSON.stringify({ userName: values.userName }),
    });
    const json = await res.json();
    if (!res.ok) {
      throw new Error(toCreateUserApiErrorMessage(res.status, json?.error));
    }
  } catch (e) {
    try {
      await deleteUser(cred.user);
      // console.error(`rollback complete: delete ${cred.user.email}`);
    } catch (rollbackError) {
      console.error("rollback failed", rollbackError);
    }
    throw e;
  }
};

export const onSubmitSignIn = async (values: SignInSchema) => {
  const auth = getFirebaseAuth();
  const email = values.email;
  const password = values.password;
  try {
    const _cred = await signInWithEmailAndPassword(auth, email, password);
  } catch (e) {
    throw new Error(toFirebaseAuthErrorMessage(e));
  }
};

export const onSubmitSignout = async () => {
  const auth = getFirebaseAuth();
  const user = auth.currentUser;

  if (user) {
    try {
      const idToken = await user.getIdToken();
      const res = await fetch("/api/users/me", {
        method: "POST",
        headers: {
          Authorization: `Bearer ${idToken}`,
        },
      });
      if (!res.ok) {
        const json = await res.json().catch(() => null);
        const message = typeof json?.error === "string" ? json.error : "unknown error";
        console.error(`token revoke failed before signout: status=${res.status}, error=${message}`);
      }
    } catch (e) {
      console.warn("token revoke failed before signout", e);
    }
  }

  await signOut(auth);
};

export const onSubmitDeleteAccount = async () => {
  const auth = getFirebaseAuth();
  const user = auth.currentUser;
  if (!user) {
    throw new Error("ログイン状態を確認できませんでした。再ログインしてください。");
  }

  const idToken = await user.getIdToken();
  try {
    await deleteUser(user);
  } catch (e) {
    throw new Error(toFirebaseAuthErrorMessage(e));
  }

  try {
    const res = await fetch("/api/users/me", {
      method: "DELETE",
      headers: {
        Authorization: `Bearer ${idToken}`,
      },
    });
    const json = await res.json().catch(() => null);
    if (!res.ok) {
      const message =
        typeof json?.error === "string" && json.error.length > 0 ? json.error : "サーバー上のデータ削除に失敗しました。";
      throw new Error(message);
    }
  } catch (e) {
    const detail = e instanceof Error ? e.message : "unknown error";
    console.error("backend delete failed after firebase delete", detail);
    throw new Error("アカウント削除は完了しましたが、サーバー上のデータ削除で一部失敗しました。時間をおいて確認してください。");
  }
};
