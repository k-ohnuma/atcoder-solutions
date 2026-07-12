import { UpdatePasswordForm } from "@/features/auth/ui/organisms/UpdatePasswordForm";
import { AuthTemplate } from "@/features/auth/ui/templates/AuthTemplate";
import { RequireAuth } from "@/features/auth/ui/templates/RequireAuth";

export default function PasswordSettingsPage() {
  return (
    <RequireAuth>
      <AuthTemplate title="パスワード変更">
        <UpdatePasswordForm />
      </AuthTemplate>
    </RequireAuth>
  );
}
