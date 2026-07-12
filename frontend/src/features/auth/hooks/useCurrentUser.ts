"use client";

import { onAuthStateChanged } from "firebase/auth";
import { useEffect, useState } from "react";
import { userApi } from "@/features/auth/api/userApi";
import { getFirebaseAuth } from "@/shared/firebase/client";

type CurrentUser = {
  status: "loading" | "guest" | "authenticated";
  isLoggedIn: boolean;
  myUserName: string | null;
};

export function useCurrentUser(): CurrentUser {
  const [currentUser, setCurrentUser] = useState<CurrentUser>({
    status: "loading",
    isLoggedIn: false,
    myUserName: null,
  });
  const auth = getFirebaseAuth();

  useEffect(() => {
    const unsub = onAuthStateChanged(auth, async (user) => {
      if (!user) {
        setCurrentUser({ status: "guest", isLoggedIn: false, myUserName: null });
        return;
      }

      try {
        const token = await user.getIdToken();
        const me = await userApi.getMe(token);
        setCurrentUser({
          status: "authenticated",
          isLoggedIn: true,
          myUserName: me.ok ? me.data.userName : null,
        });
      } catch {
        setCurrentUser({ status: "authenticated", isLoggedIn: true, myUserName: null });
      }
    });

    return () => unsub();
  }, [auth]);

  return currentUser;
}
