"use client";

import { onAuthStateChanged } from "firebase/auth";
import { useRouter } from "next/navigation";
import { ReactNode, useEffect, useState } from "react";
import { useToast } from "@/components/ui/toast";
import { getFirebaseAuth } from "@/shared/firebase/client";

type RequireAuthProps = {
  children: ReactNode;
  redirectTo?: string;
  message?: string;
};

export function RequireAuth({ children, redirectTo = "/signin", message }: RequireAuthProps) {
  const router = useRouter();
  const { toast } = useToast();
  const [canShow, setCanShow] = useState(false);
  const auth = getFirebaseAuth();

  useEffect(() => {
    let didResolveInitialAuth = false;
    const unsub = onAuthStateChanged(auth, (user) => {
      if (didResolveInitialAuth) {
        return;
      }
      didResolveInitialAuth = true;

      if (!user) {
        router.replace(redirectTo);
        if (message) {
          toast({ title: message });
        }
        return;
      }

      setCanShow(true);
    });

    return () => unsub();
  }, [auth, message, redirectTo, router, toast]);

  if (!canShow) {
    return null;
  }

  return <>{children}</>;
}
