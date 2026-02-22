import { Heart } from "lucide-react";
import Link from "next/link";
import { notFound } from "next/navigation";
import { PageContainer } from "@/components/layout/PageContainer";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { buildAtcoderProblemUrl } from "@/lib/atcoder";
import { ApiClient } from "@/lib/server/apiClient";
import { serverConfig } from "@/shared/config/backend";
import { SolutionListSortBy } from "@/shared/model/solution";

const apiClient = new ApiClient(serverConfig.appConfig.apiBaseEndpoint);

const dateTimeFormatter = new Intl.DateTimeFormat("ja-JP", {
  timeZone: "Asia/Tokyo",
  year: "numeric",
  month: "2-digit",
  day: "2-digit",
  hour: "2-digit",
  minute: "2-digit",
  second: "2-digit",
  hour12: false,
});

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
  const atcoderProblemUrl = buildAtcoderProblemUrl(problemId);
  const selectedSort: SolutionListSortBy = sortBy === "votes" ? "votes" : "latest";
  const solutionsResp = await apiClient.getSolutionsByProblemId(problemId, selectedSort);

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
          <h1 className="mb-1 text-2xl font-bold">{h1Title}</h1>
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
            <Link key={solution.id} href={`/solutions/${solution.id}`} className="block">
              <Card className="transition-colors hover:bg-accent">
                <CardContent className="p-4">
                  <div className="mb-3 flex items-start justify-between gap-3">
                    <h2 className="line-clamp-2 text-lg font-semibold leading-snug">{solution.title}</h2>
                    <Badge variant="outline" className="shrink-0 gap-1 rounded-md px-2 py-1 text-sm">
                      <Heart className="size-4" />
                      {solution.votesCount}
                    </Badge>
                  </div>

                  <div className="mb-4 flex flex-wrap items-center gap-3 text-xs text-muted-foreground">
                    <span>投稿者: {solution.userName}</span>
                    <span>{dateTimeFormatter.format(new Date(solution.createdAt))}</span>
                  </div>

                  <p className="text-sm font-medium">解説を読む</p>
                </CardContent>
              </Card>
            </Link>
          ))}
        </div>
      )}
    </PageContainer>
  );
}
