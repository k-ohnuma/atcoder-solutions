import { getAuth } from "firebase/auth";

export async function getFirebaseIdToken(): Promise<string | null> {
  const user = getAuth().currentUser;
  if (!user) return null;
  return user.getIdToken();
}
