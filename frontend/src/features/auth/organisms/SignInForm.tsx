"use client";

import * as React from "react";
import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Button } from "@/components/ui/button";
import { EmailField, PasswordField } from "../molecules";
import { css } from "styled-system/css";
import { formStyle } from "./style/Form/formStyle";

const schema = z.object({
  email: z.email({ error: "メールアドレスの形式が正しくありません" }),
  password: z.string().min(1, "パスワードを入力してください"),
});

type FormValues = z.infer<typeof schema>;

export function SignInForm() {
  const form = useForm<FormValues>({
    resolver: zodResolver(schema),
    defaultValues: {
      email: "",
      password: "",
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
    <form onSubmit={handleSubmit(onSubmit)} className={formStyle}>
      <EmailField control={control} />
      <PasswordField control={control} />

      {form.formState.errors.root?.message && (
        // <p className="rounded-md border border-red-300 p-3 text-sm text-red-700">
        <p
          className={css({
            rounded: "md",
            border: "blue",
            p: "3",
            fontSize: "sm",
          })}
        >
          {form.formState.errors.root.message}
        </p>
      )}

      <Button type="submit" disabled={isSubmitting} className="w-full">
        {isSubmitting ? "送信中..." : "新規登録"}
      </Button>
    </form>
  );
}
