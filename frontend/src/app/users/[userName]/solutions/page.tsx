import Link from "next/link";
import { notFound } from "next/navigation";
import { PageContainer } from "@/components/layout/PageContainer";
import { Button } from "@/components/ui/button";
import { SolutionSummaryCard } from "@/features/solutions/ui/molecules";
import { ApiClient } from "@/lib/server/apiClient";
import { serverConfig } from "@/shared/config/backend";
import { SolutionListSortBy } from "@/shared/model/solution";

const apiClient = new ApiClient(serverConfig.appConfig.apiBaseEndpoint);

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
  const selectedSort: SolutionListSortBy = sortBy === "votes" ? "votes" : "latest";
  const solutionsResp = await apiClient.getSolutionsByUserName(userName, selectedSort);

  const h1Title = `@${userName} の解説一覧`;

  if (!solutionsResp.ok) {
    if (solutionsResp.status === 404) {
      notFound();
    }
    throw new Error(`failed to fetch solutions by userName: status=${solutionsResp.status}, error=${solutionsResp.error}`);
  }

  const solutions = solutionsResp.data;

  return (
    <PageContainer as="div">
      <h1 className="mb-4 text-2xl font-bold">{h1Title}</h1>
      <div className="mb-6 flex items-center gap-2 text-sm">
        <Button asChild size="sm" variant={selectedSort === "latest" ? "default" : "outline"}>
          <Link href={`/users/${encodeURIComponent(userName)}/solutions?sortBy=latest`}>新着順</Link>
        </Button>
        <Button asChild size="sm" variant={selectedSort === "votes" ? "default" : "outline"}>
          <Link href={`/users/${encodeURIComponent(userName)}/solutions?sortBy=votes`}>いいね順</Link>
        </Button>
      </div>

      {solutions.length === 0 ? (
        <p className="text-sm text-muted-foreground">このユーザーの解説はまだありません。</p>
      ) : (
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
          {solutions.map((solution) => (
            <SolutionSummaryCard
              key={solution.id}
              href={`/solutions/${solution.id}`}
              title={solution.title}
              votesCount={solution.votesCount}
              createdAt={solution.createdAt}
              problemId={solution.problemId}
              problemTitle={solution.problemTitle}
            />
          ))}
        </div>
      )}
    </PageContainer>
  );
}
