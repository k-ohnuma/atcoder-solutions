import { updatePassword } from "firebase/auth";
import { getFirebaseAuth } from "@/shared/firebase/client";
import { UpdatePasswordSchema } from "../model/schema";
import { toFirebaseAuthErrorMessage } from "./errors";

export const onSubmitUpdatePassword = async (values: UpdatePasswordSchema) => {
  const auth = getFirebaseAuth();
  const user = auth.currentUser;
  if (!user) {
    throw new Error("ログイン状態を確認できませんでした。再ログインしてください。");
  }

  try {
    await updatePassword(user, values.password);
  } catch (error) {
    throw new Error(toFirebaseAuthErrorMessage(error));
  }
};
