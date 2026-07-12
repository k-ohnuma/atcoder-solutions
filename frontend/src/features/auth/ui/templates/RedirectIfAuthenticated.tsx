"use client";

import { onAuthStateChanged } from "firebase/auth";
import { useRouter } from "next/navigation";
import { ReactNode, useEffect, useState } from "react";
import { getFirebaseAuth } from "@/shared/firebase/client";

type RedirectIfAuthenticatedProps = {
  children: ReactNode;
  redirectTo?: string;
};

export function RedirectIfAuthenticated({ children, redirectTo = "/" }: RedirectIfAuthenticatedProps) {
  const router = useRouter();
  const [canShow, setCanShow] = useState(false);
  const auth = getFirebaseAuth();

  useEffect(() => {
    let didResolveInitialAuth = false;
    const unsub = onAuthStateChanged(auth, (user) => {
      if (didResolveInitialAuth) {
        return;
      }
      didResolveInitialAuth = true;

      if (user) {
        router.replace(redirectTo);
        return;
      }

      setCanShow(true);
    });

    return () => unsub();
  }, [auth, redirectTo, router]);

  if (!canShow) {
    return null;
  }

  return <>{children}</>;
}
