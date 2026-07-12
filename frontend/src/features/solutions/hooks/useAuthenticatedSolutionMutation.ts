"use client";

import { useRouter } from "next/navigation";
import { useState } from "react";
import { useToast } from "@/components/ui/toast";
import { getFirebaseIdToken } from "@/lib/client/firebaseToken";

type RunMutationParams<T> = {
  action: (token: string) => Promise<T>;
  fallbackErrorMessage: string;
  errorToastTitle: string;
};

export function useAuthenticatedSolutionMutation() {
  const router = useRouter();
  const { toast } = useToast();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const runMutation = async <T>({ action, fallbackErrorMessage, errorToastTitle }: RunMutationParams<T>) => {
    if (isSubmitting) {
      return null;
    }

    const token = await getFirebaseIdToken();
    if (!token) {
      router.push("/signin");
      return null;
    }

    setIsSubmitting(true);
    setError(null);
    try {
      return await action(token);
    } catch (error) {
      const message = error instanceof Error ? error.message : fallbackErrorMessage;
      setError(message);
      toast({ title: errorToastTitle, description: message, variant: "error" });
      return null;
    } finally {
      setIsSubmitting(false);
    }
  };

  return {
    isSubmitting,
    error,
    setError,
    runMutation,
  };
}
