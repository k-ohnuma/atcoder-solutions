import Link from "next/link";
import { notFound } from "next/navigation";
import { PageContainer } from "@/components/layout/PageContainer";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { SolutionSummaryCard } from "@/features/solutions/ui/molecules";
import { buildAtcoderProblemUrl } from "@/lib/atcoder";
import { ApiClient } from "@/lib/server/apiClient";
import { serverConfig } from "@/shared/config/backend";
import { SolutionListSortBy } from "@/shared/model/solution";

const apiClient = new ApiClient(serverConfig.appConfig.apiBaseEndpoint);

type PageProps = {
  params: Promise<{
    problemId: string;
  }>;
  searchParams: Promise<{
    sortBy?: string;
  }>;
};

function difficultyBadgeClass(difficulty?: number | null): string {
  if (difficulty === null || difficulty === undefined) {
    return "";
  }
  if (difficulty < 400) return "border-transparent bg-[#808080] text-white";
  if (difficulty < 800) return "border-transparent bg-[#804000] text-white";
  if (difficulty < 1200) return "border-transparent bg-[#008000] text-white";
  if (difficulty < 1600) return "border-transparent bg-[#00C0C0] text-white";
  if (difficulty < 2000) return "border-transparent bg-[#0000FF] text-white";
  if (difficulty < 2400) return "border-transparent bg-[#C0C000] text-black";
  if (difficulty < 2800) return "border-transparent bg-[#FF8000] text-white";
  return "border-transparent bg-[#FF0000] text-white";
}

export default async function ProblemSolutionsPage({ params, searchParams }: PageProps) {
  const { problemId } = await params;
  const { sortBy } = await searchParams;
  const atcoderProblemUrl = buildAtcoderProblemUrl(problemId);
  const selectedSort: SolutionListSortBy = sortBy === "votes" ? "votes" : "latest";
  const solutionsResp = await apiClient.getSolutionsByProblemId(problemId, selectedSort);
  const contestCode = problemId.split("_")[0] ?? "";
  const contestProblems = contestCode ? await apiClient.getProblemsByContest(contestCode) : [];
  const difficulty = contestProblems.find((problem) => problem.id === problemId)?.difficulty ?? null;

  const h1Title = `${problemId} の解説一覧`;

  if (!solutionsResp.ok) {
    if (solutionsResp.status === 404) {
      notFound();
    }
    throw new Error(`failed to fetch solutions by problemId: status=${solutionsResp.status}, error=${solutionsResp.error}`);
  }

  const solutions = solutionsResp.data;

  return (
    <PageContainer as="div">
      <div className="mb-4 flex flex-wrap items-center justify-between gap-2">
        <div>
          <div className="mb-1 flex items-center gap-2">
            <h1 className="text-2xl font-bold">{h1Title}</h1>
            <Badge
              variant="outline"
              className={`px-1.5 py-0 text-[11px] leading-tight ${difficultyBadgeClass(difficulty)}`}
              title={`Difficulty: ${difficulty ?? "N/A"}`}
            >
              {difficulty ?? "N/A"}
            </Badge>
          </div>
          <Link href={atcoderProblemUrl} target="_blank" rel="noreferrer" className="inline-block text-sm hover:underline">
            AtCoder問題ページ
          </Link>
        </div>
        <Button asChild variant="outline" size="sm">
          <Link href={`/solutions/create?problemId=${encodeURIComponent(problemId)}`}>この問題の記事を書く</Link>
        </Button>
      </div>
      <div className="mb-6 flex items-center gap-2 text-sm">
        <Button asChild size="sm" variant={selectedSort === "latest" ? "default" : "outline"}>
          <Link href={`/problems/${problemId}?sortBy=latest`}>新着順</Link>
        </Button>
        <Button asChild size="sm" variant={selectedSort === "votes" ? "default" : "outline"}>
          <Link href={`/problems/${problemId}?sortBy=votes`}>いいね順</Link>
        </Button>
      </div>

      {solutions.length === 0 ? (
        <p className="text-sm text-muted-foreground">この問題の解説はまだありません。</p>
      ) : (
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
          {solutions.map((solution) => (
            <SolutionSummaryCard
              key={solution.id}
              href={`/solutions/${solution.id}`}
              title={solution.title}
              votesCount={solution.votesCount}
              createdAt={solution.createdAt}
              userName={solution.userName}
              footerLabel="解説を読む"
            />
          ))}
        </div>
      )}
    </PageContainer>
  );
}
