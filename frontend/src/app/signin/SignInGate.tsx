"use client";

import { onAuthStateChanged } from "firebase/auth";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { getFirebaseAuth } from "@/shared/firebase/client";

export function SignInGate({ children }: { children: React.ReactNode }) {
  const router = useRouter();
  const [canShow, setCanShow] = useState(false);
  const auth = getFirebaseAuth();

  useEffect(() => {
    const unsub = onAuthStateChanged(auth, (user) => {
      if (user) {
        router.replace("/");
        return;
      }
      setCanShow(true);
    });
    return () => {
      unsub();
    };
  }, [auth, router]);

  if (!canShow) {
    return null;
  }

  return <>{children}</>;
}
