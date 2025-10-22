"use client";

import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Button } from "@/components/ui/button";
import { EmailField, PasswordField } from "../molecules";
import { css } from "styled-system/css";
import { formStyle } from "./style/Form/formStyle";
import { signInSchema, SignInSchema } from "../../model/schema";
import { onSubmitSignUp } from "../../lib/submit";

export function SignInForm() {
  const form = useForm<SignInSchema>({
    resolver: zodResolver(signInSchema),
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

  const onSubmit = async (values: SignInSchema) => {
    console.log(values);
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className={formStyle}>
      <EmailField control={control} />
      <PasswordField control={control} />

      {form.formState.errors.root?.message && (
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
