import { RequireAuth } from "@/features/auth/ui/templates/RequireAuth";
import { CreateSolutionTemplate } from "@/features/solutions/ui/templates/CreateSolutionTemplate";

export default function CreateSolutionPage() {
  return (
    <RequireAuth message="ログインしてください">
      <CreateSolutionTemplate />
    </RequireAuth>
  );
}
