"use client";

import { Button } from "@/components/ui/button";
import { Form } from "@/components/ui/form";
import { useSignUpForm } from "../../hooks/useSignUpForm";
import { AuthFormError } from "../molecules/AuthFormError";
import { ConfirmPasswordField } from "../molecules/ConfirmPasswordField";
import { EmailField } from "../molecules/EmailField";
import { PasswordField } from "../molecules/PasswordField";
import { TurnstileField } from "@/shared/turnstile/TurnstileField";
import { UserNameField } from "../molecules/UserNameField";
import { clientConfig } from "@/shared/config/client";

export function SignUpForm() {
  const { form, submit, turnstileToken, setTurnstileToken } = useSignUpForm();
  const {
    control,
    formState: { isSubmitting },
  } = form;

  return (
    <Form {...form}>
      <form onSubmit={submit} className="grid gap-4">
        <UserNameField control={control} />
        <EmailField control={control} />
        <PasswordField control={control} />
        <ConfirmPasswordField control={control} />
        <TurnstileField onTokenChangeAction={setTurnstileToken} />

        <AuthFormError message={form.formState.errors.root?.message} />

        <Button
          type="submit"
          disabled={isSubmitting || (clientConfig.turnstileConfig.enabled && !turnstileToken)}
          className="w-full"
        >
          {isSubmitting ? "送信中..." : "新規登録"}
        </Button>
      </form>
    </Form>
  );
}
