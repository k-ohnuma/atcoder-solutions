import { createUserWithEmailAndPassword, deleteUser, signOut, type UserCredential } from "firebase/auth";
import { userApi } from "@/features/auth/api/userApi";
import { getFirebaseAuth } from "@/shared/firebase/client";
import { SignUpSchema } from "../model/schema";
import { toCreateUserApiErrorMessage, toFirebaseAuthErrorMessage } from "./errors";

async function rollbackFirebaseUser(cred: UserCredential) {
  const auth = getFirebaseAuth();
  try {
    await deleteUser(cred.user);
    console.error(`rollback complete: delete ${cred.user.email}`);
  } catch (rollbackError) {
    console.error("rollback failed", rollbackError);
  } finally {
    try {
      await signOut(auth);
    } catch (signOutError) {
      console.error("sign out after signup failure failed", signOutError);
    }
  }
}

export const onSubmitSignUp = async (values: SignUpSchema) => {
  const auth = getFirebaseAuth();
  let cred: UserCredential;
  try {
    cred = await createUserWithEmailAndPassword(auth, values.email, values.password);
  } catch (error) {
    throw new Error(toFirebaseAuthErrorMessage(error));
  }

  try {
    const idToken = await cred.user.getIdToken();
    const created = await userApi.create(values.userName, idToken);
    if (!created.ok) {
      throw new Error(toCreateUserApiErrorMessage(created.status, created.error));
    }
  } catch (error) {
    await rollbackFirebaseUser(cred);
    throw error;
  }
};
