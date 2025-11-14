import type { Metadata } from "next";
import { SignInForm } from "@/features/auth/ui/organisms";
import { AuthTemplate } from "@/features/auth/ui/templates";

export const metadata: Metadata = { title: "ログイン" };

export default function SignUpPage() {
  return (
    <AuthTemplate title="ログイン">
      <SignInForm />
    </AuthTemplate>
  );
}
