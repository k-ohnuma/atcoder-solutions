"use client";

import { Button } from "@/components/ui/button";
import { Form } from "@/components/ui/form";
import { useSignInForm } from "../../hooks/useSignInForm";
import { AuthFormError } from "../molecules/AuthFormError";
import { EmailField } from "../molecules/EmailField";
import { PasswordField } from "../molecules/PasswordField";

export function SignInForm() {
  const { form, submit } = useSignInForm();
  const {
    control,
    formState: { isSubmitting },
  } = form;

  return (
    <Form {...form}>
      <form onSubmit={submit} className="grid gap-4">
        <AuthFormError message={form.formState.errors.root?.message} />

        <EmailField control={control} />
        <PasswordField control={control} />

        <Button type="submit" disabled={isSubmitting} className="w-full">
          {isSubmitting ? "送信中..." : "ログイン"}
        </Button>
      </form>
    </Form>
  );
}
