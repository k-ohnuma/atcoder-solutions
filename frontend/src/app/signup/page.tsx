import type { Metadata } from "next";
import { SignUpForm } from "@/features/auth/ui/organisms/SignUpForm";
import { AuthTemplate } from "@/features/auth/ui/templates/AuthTemplate";
import { RedirectIfAuthenticated } from "@/features/auth/ui/templates/RedirectIfAuthenticated";

export const metadata: Metadata = { title: "新規登録" };

export default function SignUpPage() {
  return (
    <RedirectIfAuthenticated>
      <AuthTemplate title="新規登録">
        <SignUpForm />
      </AuthTemplate>
    </RedirectIfAuthenticated>
  );
}
