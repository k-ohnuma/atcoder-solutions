import { SignInForm } from "@/features/auth/organisms";
import { AuthTemplate } from "@/features/auth/templates";
import type { Metadata } from "next";

export const metadata: Metadata = { title: "ログイン" };

export default function SignUpPage() {
  return (
    <AuthTemplate title="ログイン">
      <SignInForm />
    </AuthTemplate>
  );
}
