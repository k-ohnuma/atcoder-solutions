import { signInWithEmailAndPassword, signOut } from "firebase/auth";
import { userApi } from "@/features/auth/api/userApi";
import { getFirebaseAuth } from "@/shared/firebase/client";
import { SignInSchema } from "../model/schema";
import { RequiresRecentLoginError, toDeleteAccountApiErrorMessage, toFirebaseAuthErrorMessage } from "./errors";

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
  const deleted = await userApi.deleteMe(idToken);
  if (!deleted.ok) {
    if (deleted.status === 401 && deleted.errorCode === "REQUIRES_RECENT_LOGIN") {
      throw new RequiresRecentLoginError();
    }
    throw new Error(toDeleteAccountApiErrorMessage(deleted.status, deleted.error, deleted.errorCode));
  }

  await signOut(auth);
};
