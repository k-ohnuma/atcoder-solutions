import { notFound } from "next/navigation";
import { normalizeSolutionListSort } from "@/features/solutions/model/solutionList";
import { UserSolutionsTemplate } from "@/features/solutions/ui/templates/UserSolutionsTemplate";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";

const solutionRepository = new SolutionRepositoryImpl();

type PageProps = {
  params: Promise<{
    userName: string;
  }>;
  searchParams: Promise<{
    sortBy?: string;
  }>;
};

export default async function UserSolutionsPage({ params, searchParams }: PageProps) {
  const { userName } = await params;
  const { sortBy } = await searchParams;
  const selectedSort = normalizeSolutionListSort(sortBy);
  const solutionsResp = await solutionRepository.getByUserName(userName, selectedSort);

  if (!solutionsResp.ok) {
    if (solutionsResp.status === 404) {
      notFound();
    }
    throw new Error(`failed to fetch solutions by userName: status=${solutionsResp.status}, error=${solutionsResp.error}`);
  }

  const solutions = solutionsResp.data;

  return <UserSolutionsTemplate userName={userName} selectedSort={selectedSort} solutions={solutions} />;
}
