import { getFirebaseAuth } from "@/shared/firebase/client";
import { SignInSchema, SignUpSchema } from "../model/schema";
import {
  createUserWithEmailAndPassword,
  deleteUser,
  signInWithEmailAndPassword,
  signOut,
  UserCredential,
} from "firebase/auth";

export const onSubmitSignUp = async (values: SignUpSchema) => {
  let cred: UserCredential | undefined;
  const auth = getFirebaseAuth();
  const email = values.email;
  const password = values.password;
  // とりあえずfirebaseの時点でこけたらそこで終了させる
  try {
    cred = await createUserWithEmailAndPassword(auth, email, password);
  } catch (e: any) {
    throw e;
  }

  if (!cred) throw new Error("internal server error: cred is undefined");
  // firebaseが成功して、DBへの格納がこけたらfirebaseもロールバックしなきゃ
  try {
    const idToken = await cred.user.getIdToken();
    const res = await fetch("/api/users", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${idToken}`,
      },
      body: JSON.stringify({ userName: values.userName, color: values.color }),
    });
    const json = await res.json();
    if (!res.ok) throw new Error(json.error);
  } catch (e) {
    await deleteUser(cred.user);
    console.error(`rollback complete: delete ${cred.user.email}`);
    throw e;
  }
};

export const onSubmitSignIn = async (values: SignInSchema) => {
  const auth = getFirebaseAuth();
  const email = values.email;
  const password = values.password;
  try {
    const _cred = await signInWithEmailAndPassword(auth, email, password);
  } catch (e: any) {
    throw e;
  }
};

export const onSubmitSignout = async() => {
  const auth = getFirebaseAuth();
  await signOut(auth)
}
