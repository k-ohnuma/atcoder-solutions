import { getFirebaseAuth } from "@/shared/firebase/client";
import { SignUpSchema } from "../model/schema";
import { createUserWithEmailAndPassword } from "firebase/auth";

export const onSubmitSignUp = async (values: SignUpSchema) => {
  const auth = getFirebaseAuth();

  const email = values.email;
  const password = values.password;

  const cred = await createUserWithEmailAndPassword(auth, email, password);
  console.log(cred.user.uid);
  return cred.user.uid;
};
