"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useRouter } from "next/navigation";
import { useForm } from "react-hook-form";
import { Button } from "@/components/ui/button";
import { Form } from "@/components/ui/form";
import { onSubmitSignUp } from "../../lib/submit";
import { SignUpSchema, signUpSchema } from "../../model/schema";
import { ConfirmPasswordField, EmailField, PasswordField, UserNameField } from "../molecules";

export function SignUpForm() {
  const form = useForm<SignUpSchema>({
    resolver: zodResolver(signUpSchema),
    defaultValues: {
      userName: "",
      email: "",
      password: "",
      confirm: "",
    },
  });

  const {
    handleSubmit,
    control,
    formState: { isSubmitting },
  } = form;

  const router = useRouter();

  return (
    <Form {...form}>
      <form
        onSubmit={handleSubmit(async (value) => {
          try {
            await onSubmitSignUp(value);
            router.push("/");
          } catch (e: any) {
            form.setError("root", { message: e.message });
          }
        })}
        className="grid gap-4"
      >
        <UserNameField control={control} />
        <EmailField control={control} />
        <PasswordField control={control} />
        <ConfirmPasswordField control={control} />

        {form.formState.errors.root?.message && (
          <p className="rounded-md border border-destructive/50 bg-destructive/5 p-3 text-sm text-destructive">
            {form.formState.errors.root.message}
          </p>
        )}

        <Button type="submit" disabled={isSubmitting} className="w-full">
          {isSubmitting ? "送信中..." : "新規登録"}
        </Button>
      </form>
    </Form>
  );
}
