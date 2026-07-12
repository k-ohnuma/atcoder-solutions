import { RecentSolutionsTemplate } from "@/features/solutions/ui/templates/RecentSolutionsTemplate";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";

export const dynamic = "force-dynamic";

const solutionRepository = new SolutionRepositoryImpl();

export default async function RecentSolutionsPage() {
  const solutionsResp = await solutionRepository.getLatest(50);

  if (!solutionsResp.ok) {
    throw new Error(`failed to fetch latest solutions: status=${solutionsResp.status}, error=${solutionsResp.error}`);
  }

  return <RecentSolutionsTemplate solutions={solutionsResp.data} />;
}
