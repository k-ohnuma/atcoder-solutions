"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useRouter } from "next/navigation";
import { useForm } from "react-hook-form";
import { getAuthFormErrorMessage } from "@/features/auth/lib/error";
import { onSubmitSignIn } from "@/features/auth/lib/submit";
import { SignInSchema, signInSchema } from "@/features/auth/model/schema";

export function useSignInForm() {
  const router = useRouter();
  const form = useForm<SignInSchema>({
    resolver: zodResolver(signInSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  });

  const submit = form.handleSubmit(async (values) => {
    try {
      await onSubmitSignIn(values);
      router.push("/");
    } catch (error) {
      form.setError("root", { message: getAuthFormErrorMessage(error) });
    }
  });

  return {
    form,
    submit,
  };
}
