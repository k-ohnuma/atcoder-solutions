"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useRouter } from "next/navigation";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { getAuthFormErrorMessage } from "@/features/auth/lib/errors";
import { onSubmitSignUp } from "@/features/auth/lib/signup";
import { SignUpSchema, signUpSchema } from "@/features/auth/model/schema";

export function useSignUpForm() {
  const router = useRouter();
  const [turnstileToken, setTurnstileToken] = useState<string | null>(null);
  const [turnstileResetKey, setTurnstileResetKey] = useState(0);
  const form = useForm<SignUpSchema>({
    resolver: zodResolver(signUpSchema),
    defaultValues: {
      userName: "",
      email: "",
      password: "",
      confirm: "",
    },
  });

  const submit = form.handleSubmit(async (values) => {
    try {
      await onSubmitSignUp(values, turnstileToken);
      router.push("/");
    } catch (error) {
      setTurnstileToken(null);
      setTurnstileResetKey((current) => current + 1);
      form.setError("root", { message: getAuthFormErrorMessage(error) });
    }
  });

  return {
    form,
    submit,
    turnstileToken,
    setTurnstileToken,
    turnstileResetKey,
  };
}
