"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { Button } from "@/components/ui/button";
import { Form } from "@/components/ui/form";
import { useToast } from "@/components/ui/toast";
import { onSubmitUpdatePassword } from "../../lib/submit";
import { UpdatePasswordSchema, updatePasswordSchema } from "../../model/schema";
import { ConfirmPasswordField, PasswordField } from "../molecules";

export function UpdatePasswordForm() {
  const { toast } = useToast();
  const form = useForm<UpdatePasswordSchema>({
    resolver: zodResolver(updatePasswordSchema),
    defaultValues: {
      password: "",
      confirm: "",
    },
  });

  const {
    handleSubmit,
    control,
    formState: { isSubmitting },
    reset,
  } = form;

  return (
    <Form {...form}>
      <form
        onSubmit={handleSubmit(async (value) => {
          try {
            await onSubmitUpdatePassword(value);
            reset();
            toast({ title: "パスワードを更新しました" });
          } catch (e: any) {
            form.setError("root", { message: e.message });
          }
        })}
        className="grid gap-4"
      >
        <PasswordField control={control} />
        <ConfirmPasswordField control={control} />

        {form.formState.errors.root?.message && (
          <p className="rounded-md border border-destructive/50 bg-destructive/5 p-3 text-sm text-destructive">
            {form.formState.errors.root.message}
          </p>
        )}

        <Button type="submit" disabled={isSubmitting} className="w-full">
          {isSubmitting ? "更新中..." : "パスワードを更新"}
        </Button>
      </form>
    </Form>
  );
}
