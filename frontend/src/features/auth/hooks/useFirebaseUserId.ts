"use client";

import { onAuthStateChanged } from "firebase/auth";
import { useEffect, useState } from "react";
import { getFirebaseAuth } from "@/shared/firebase/client";

export function useFirebaseUserId() {
  const auth = getFirebaseAuth();
  const [userId, setUserId] = useState<string | null>(null);

  useEffect(() => {
    const unsub = onAuthStateChanged(auth, (user) => {
      setUserId(user?.uid ?? null);
    });

    return () => unsub();
  }, [auth]);

  return userId;
}
