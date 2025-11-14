import type { Metadata } from "next";
import { SignUpForm } from "@/features/auth/ui/organisms";
import { AuthTemplate } from "@/features/auth/ui/templates";

export const metadata: Metadata = { title: "新規登録" };

export default function SignUpPage() {
  return (
    <AuthTemplate title="新規登録">
      <SignUpForm />
    </AuthTemplate>
  );
}
