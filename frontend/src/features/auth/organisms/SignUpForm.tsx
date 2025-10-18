"use client";

import * as React from "react";
import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Button } from "@/components/ui/button";
import {
  ColorField,
  ConfirmPasswordField,
  EmailField,
  PasswordField,
  UserNameField,
} from "../molecules";

const schema = z
  .object({
    userName: z.string().min(1, "ユーザー名を入力してください"),
    email: z.email({ error: "メールアドレスの形式が正しくありません" }),
    password: z.string().min(1, "パスワードを入力してください"),
    confirm: z.string(),
    color: z.string().min(1, "選択してください"),
  })
  .refine((v) => v.password === v.confirm, {
    path: ["confirm"],
    message: "パスワードが一致しません",
  });

type FormValues = z.infer<typeof schema>;

export function SignUpForm() {
  const form = useForm<FormValues>({
    resolver: zodResolver(schema),
    defaultValues: {
      userName: "",
      email: "",
      password: "",
      confirm: "",
      color: "",
    },
  });
  const {
    handleSubmit,
    control,
    formState: { isSubmitting },
  } = form;

  const onSubmit = async (values: FormValues) => {
    console.log(values);
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="grid gap-4">
      <UserNameField control={control} />
      <EmailField control={control} />
      <PasswordField control={control} />
      <ConfirmPasswordField control={control} />
      <ColorField control={control} />

      {form.formState.errors.root?.message && (
        <p className="rounded-md border border-red-300 p-3 text-sm text-red-700">
          {form.formState.errors.root.message}
        </p>
      )}

      <Button type="submit" disabled={isSubmitting} className="w-full">
        {isSubmitting ? "送信中..." : "新規登録"}
      </Button>
    </form>
  );
}
