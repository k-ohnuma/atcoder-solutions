import { deleteUser, signInWithEmailAndPassword, signOut } from "firebase/auth";
import { userApi } from "@/features/auth/api/userApi";
import { getFirebaseAuth } from "@/shared/firebase/client";
import { SignInSchema } from "../model/schema";
import { toFirebaseAuthErrorMessage } from "./errors";

export const onSubmitSignIn = async (values: SignInSchema) => {
  const auth = getFirebaseAuth();
  try {
    await signInWithEmailAndPassword(auth, values.email, values.password);
  } catch (error) {
    throw new Error(toFirebaseAuthErrorMessage(error));
  }
};

export const onSubmitSignout = async () => {
  const auth = getFirebaseAuth();
  const user = auth.currentUser;

  if (user) {
    try {
      const idToken = await user.getIdToken();
      const revoked = await userApi.revokeMe(idToken);
      if (!revoked.ok) {
        console.error(`token revoke failed before signout: status=${revoked.status}, error=${revoked.error}`);
      }
    } catch (error) {
      console.warn("token revoke failed before signout", error);
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
  } catch (error) {
    throw new Error(toFirebaseAuthErrorMessage(error));
  }

  try {
    const deleted = await userApi.deleteMe(idToken);
    if (!deleted.ok) {
      const message =
        typeof deleted.error === "string" && deleted.error.length > 0
          ? deleted.error
          : "サーバー上のデータ削除に失敗しました。";
      throw new Error(message);
    }
  } catch (error) {
    const detail = error instanceof Error ? error.message : "unknown error";
    console.error("backend delete failed after firebase delete", detail);
    throw new Error("アカウント削除は完了しましたが、サーバー上のデータ削除で一部失敗しました。時間をおいて確認してください。");
  }
};
