import { notFound } from "next/navigation";
import { ProblemSolutionsTemplate } from "@/features/problems/ui/templates/ProblemSolutionsTemplate";
import { normalizeSolutionListSort } from "@/features/solutions/model/solutionList";
import { ProblemRepositoryImpl } from "@/server/infrastructure/repository/problemRepository";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";

const problemRepository = new ProblemRepositoryImpl();
const solutionRepository = new SolutionRepositoryImpl();

type PageProps = {
  params: Promise<{
    problemId: string;
  }>;
  searchParams: Promise<{
    sortBy?: string;
  }>;
};

export default async function ProblemSolutionsPage({ params, searchParams }: PageProps) {
  const { problemId } = await params;
  const { sortBy } = await searchParams;
  const selectedSort = normalizeSolutionListSort(sortBy);
  const [problemResp, solutionsResp] = await Promise.all([
    problemRepository.getProblemById(problemId),
    solutionRepository.getByProblemId(problemId, selectedSort),
  ]);

  if (!problemResp.ok) {
    if (problemResp.status === 404) {
      notFound();
    }
    throw new Error(`failed to fetch problem: status=${problemResp.status}, error=${problemResp.error}`);
  }

  const problem = problemResp.data;

  if (!solutionsResp.ok) {
    if (solutionsResp.status === 404) {
      notFound();
    }
    throw new Error(`failed to fetch solutions by problemId: status=${solutionsResp.status}, error=${solutionsResp.error}`);
  }

  const solutions = solutionsResp.data;

  return <ProblemSolutionsTemplate problem={problem} selectedSort={selectedSort} solutions={solutions} />;
}
