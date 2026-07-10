"use client";

import { onAuthStateChanged } from "firebase/auth";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { useToast } from "@/components/ui/toast";
import { CreateSolutionTemplate } from "@/features/solutions/ui/templates/CreateSolutionTemplate";
import { getFirebaseAuth } from "@/shared/firebase/client";

export default function CreateSolutionPage() {
  const router = useRouter();
  const [isAllowed, setIsAllowed] = useState(false);
  const { toast } = useToast();
  const auth = getFirebaseAuth();

  useEffect(() => {
    const unsub = onAuthStateChanged(auth, (user) => {
      if (!user) {
        router.replace("/signin");
        toast({ title: "ログインしてください" });
        return;
      }
      setIsAllowed(true);
    });

    return () => {
      unsub();
    };
  }, [auth, router, toast]);

  if (!isAllowed) {
    return null;
  }

  return <CreateSolutionTemplate />;
}
