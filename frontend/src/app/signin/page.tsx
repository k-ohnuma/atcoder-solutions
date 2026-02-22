import type { Metadata } from "next";
import { SignInForm } from "@/features/auth/ui/organisms";
import { AuthTemplate } from "@/features/auth/ui/templates";
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
