import type { Metadata } from "next";
import { SignInForm } from "@/features/auth/ui/organisms/SignInForm";
import { AuthTemplate } from "@/features/auth/ui/templates/AuthTemplate";
import { RedirectIfAuthenticated } from "@/features/auth/ui/templates/RedirectIfAuthenticated";

export const metadata: Metadata = { title: "ログイン" };

export default function SignUpPage() {
  return (
    <RedirectIfAuthenticated>
      <AuthTemplate title="ログイン">
        <SignInForm />
      </AuthTemplate>
    </RedirectIfAuthenticated>
  );
}
