"use client";

import * as React from "react";
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
import { formStyle } from "./style/Form/formStyle";
import { signUpSchema, SignUpSchema } from "../../model/schema";
import { onSubmitSignUp } from "../../lib/submit";

export function SignUpForm() {
  const form = useForm<SignUpSchema>({
    resolver: zodResolver(signUpSchema),
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

  return (
    <form onSubmit={handleSubmit(onSubmitSignUp)} className={formStyle}>
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
