import type { Metadata } from "next";
import { SignInForm } from "@/features/auth/ui/organisms/SignInForm";
import { AuthTemplate } from "@/features/auth/ui/templates/AuthTemplate";
import { SignInGate } from "./SignInGate";

export const metadata: Metadata = { title: "ログイン" };

export default function SignUpPage() {
  return (
    <SignInGate>
      <AuthTemplate title="ログイン">
        <SignInForm />
      </AuthTemplate>
    </SignInGate>
  );
}
