import { createUserWithEmailAndPassword, deleteUser, signInWithEmailAndPassword, signOut } from "firebase/auth";
import { getFirebaseAuth } from "@/shared/firebase/client";
import { SignInSchema, SignUpSchema } from "../model/schema";

export const onSubmitSignUp = async (values: SignUpSchema) => {
  const auth = getFirebaseAuth();
  const email = values.email;
  const password = values.password;
  // とりあえずfirebaseの時点でこけたらそこで終了させる
  const cred = await createUserWithEmailAndPassword(auth, email, password).catch((_e) => undefined);
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
      body: JSON.stringify({ userName: values.userName }),
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
  const _cred = await signInWithEmailAndPassword(auth, email, password);
};

export const onSubmitSignout = async () => {
  const auth = getFirebaseAuth();
  await signOut(auth);
};
