"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useRouter } from "next/navigation";
import { useForm } from "react-hook-form";
import { Button } from "@/components/ui/button";
import { Form } from "@/components/ui/form";
import { onSubmitSignIn } from "../../lib/submit";
import { SignInSchema, signInSchema } from "../../model/schema";
import { EmailField, PasswordField } from "../molecules";

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

  const router = useRouter();

  return (
    <Form {...form}>
      <form
        onSubmit={handleSubmit(async (value) => {
          try {
            await onSubmitSignIn(value);
            router.push("/");
          } catch (e: any) {
            form.setError("root", { message: e.message });
          }
        })}
        className="grid gap-4"
      >
        {form.formState.errors.root?.message && (
          <p className="rounded-md border border-destructive/50 bg-destructive/5 p-3 text-sm text-destructive">
            {form.formState.errors.root.message}
          </p>
        )}

        <EmailField control={control} />
        <PasswordField control={control} />

        <Button type="submit" disabled={isSubmitting} className="w-full">
          {isSubmitting ? "送信中..." : "ログイン"}
        </Button>
      </form>
    </Form>
  );
}
