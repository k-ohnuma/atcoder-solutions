"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { useToast } from "@/components/ui/toast";
import { getAuthFormErrorMessage } from "@/features/auth/lib/error";
import { onSubmitUpdatePassword } from "@/features/auth/lib/password";
import { UpdatePasswordSchema, updatePasswordSchema } from "@/features/auth/model/schema";

export function useUpdatePasswordForm() {
  const { toast } = useToast();
  const form = useForm<UpdatePasswordSchema>({
    resolver: zodResolver(updatePasswordSchema),
    defaultValues: {
      password: "",
      confirm: "",
    },
  });

  const submit = form.handleSubmit(async (values) => {
    try {
      await onSubmitUpdatePassword(values);
      form.reset();
      toast({ title: "パスワードを更新しました" });
    } catch (error) {
      form.setError("root", { message: getAuthFormErrorMessage(error) });
    }
  });

  return {
    form,
    submit,
  };
}
