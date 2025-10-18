import { SignUpForm } from "@/features/auth/organisms";
import { AuthTemplate } from "@/features/auth/templates";
import type { Metadata } from "next";

export const metadata: Metadata = { title: "新規登録" };

export default function SignUpPage() {
  return (
    <AuthTemplate title="新規登録">
      <SignUpForm />
    </AuthTemplate>
  );
}
