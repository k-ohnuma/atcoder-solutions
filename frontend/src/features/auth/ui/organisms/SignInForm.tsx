"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useRouter } from "next/navigation";
import { useForm } from "react-hook-form";
import { css } from "styled-system/css";
import { Button } from "@/components/ui/button";
import { onSubmitSignIn } from "../../lib/submit";
import { SignInSchema, signInSchema } from "../../model/schema";
import { EmailField, PasswordField } from "../molecules";
import { formStyle } from "./style/Form/formStyle";

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
    <form
      onSubmit={handleSubmit(async (value) => {
        try {
          await onSubmitSignIn(value);
          router.push("/");
        } catch (e: any) {
          form.setError("root", { message: e.message });
        }
      })}
      className={formStyle}
    >
      {form.formState.errors.root?.message && (
        <p
          className={css({
            borderColor: "red.300",
            borderStyle: "solid",
            borderWidth: "thick",
            p: "3",
            color: "red.700",
            fontSize: "sm",
          })}
        >
          {form.formState.errors.root.message}
        </p>
      )}
      <EmailField control={control} />
      <PasswordField control={control} />

      <Button type="submit" disabled={isSubmitting} className="w-full">
        {isSubmitting ? "送信中..." : "ログイン"}
      </Button>
    </form>
  );
}
