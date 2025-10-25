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
import { css } from "styled-system/css";
import { useRouter } from "next/navigation";

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
  const router = useRouter();

  return (
    <form onSubmit={handleSubmit(async(value) => {
      try {
        await onSubmitSignUp(value);
        router.push('/');
      } catch(e: any) {
        form.setError("root", {message: e.message})
      }
    })} className={formStyle}>
      <UserNameField control={control} />
      <EmailField control={control} />
      <PasswordField control={control} />
      <ConfirmPasswordField control={control} />
      <ColorField control={control} />

      {form.formState.errors.root?.message && (
        <p className={css({
          borderColor: 'red.300',
          borderStyle: 'solid',
          borderWidth: 'thick',
          p: '3',
          color: 'red.700',
          fontSize: 'sm'
        })}>
          {form.formState.errors.root.message}
        </p>
      )}

      <Button type="submit" disabled={isSubmitting} className="w-full">
        {isSubmitting ? "送信中..." : "新規登録"}
      </Button>
    </form>
  );
}
