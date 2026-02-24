"use client";

import { onAuthStateChanged } from "firebase/auth";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { UpdatePasswordForm } from "@/features/auth/ui/organisms";
import { AuthTemplate } from "@/features/auth/ui/templates";
import { getFirebaseAuth } from "@/shared/firebase/client";

export default function PasswordSettingsPage() {
  const router = useRouter();
  const [isAllowed, setIsAllowed] = useState(false);
  const auth = getFirebaseAuth();

  useEffect(() => {
    const unsub = onAuthStateChanged(auth, (user) => {
      if (!user) {
        router.replace("/signin");
        return;
      }
      setIsAllowed(true);
    });

    return () => {
      unsub();
    };
  }, [auth, router]);

  if (!isAllowed) {
    return null;
  }

  return (
    <AuthTemplate title="パスワード変更">
      <UpdatePasswordForm />
    </AuthTemplate>
  );
}
