"use client";

import { Button } from "@/components/ui/button";
import { Form } from "@/components/ui/form";
import { useUpdatePasswordForm } from "../../hooks/useUpdatePasswordForm";
import { AuthFormError } from "../molecules/AuthFormError";
import { ConfirmPasswordField } from "../molecules/ConfirmPasswordField";
import { PasswordField } from "../molecules/PasswordField";

export function UpdatePasswordForm() {
  const { form, submit } = useUpdatePasswordForm();
  const {
    control,
    formState: { isSubmitting },
  } = form;

  return (
    <Form {...form}>
      <form onSubmit={submit} className="grid gap-4">
        <PasswordField control={control} />
        <ConfirmPasswordField control={control} />

        <AuthFormError message={form.formState.errors.root?.message} />

        <Button type="submit" disabled={isSubmitting} className="w-full">
          {isSubmitting ? "更新中..." : "パスワードを更新"}
        </Button>
      </form>
    </Form>
  );
}
