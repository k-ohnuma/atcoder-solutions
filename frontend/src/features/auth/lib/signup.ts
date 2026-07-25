import { signInWithEmailAndPassword } from "firebase/auth";
import { userApi } from "@/features/auth/api/userApi";
import { getFirebaseAuth } from "@/shared/firebase/client";
import { SignUpSchema } from "../model/schema";
import { toCreateUserApiErrorMessage, toFirebaseAuthErrorMessage } from "./errors";

export const onSubmitSignUp = async (values: SignUpSchema) => {
  const created = await userApi.create({
    userName: values.userName,
    email: values.email,
    password: values.password,
  });
  if (!created.ok) {
    throw new Error(toCreateUserApiErrorMessage(created.status, created.error, created.errorCode));
  }

  try {
    await signInWithEmailAndPassword(getFirebaseAuth(), values.email, values.password);
  } catch (error) {
    throw new Error(toFirebaseAuthErrorMessage(error));
  }
};
