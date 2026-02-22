import { Heart } from "lucide-react";
import Link from "next/link";
import { PageContainer } from "@/components/layout/PageContainer";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
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
    return (
      <PageContainer as="div">
        <h1 className="mb-3 text-2xl font-bold">{h1Title}</h1>
        <p className="text-sm text-destructive">
          解説一覧の取得に失敗しました。status: {solutionsResp.status}, error: {solutionsResp.error}
        </p>
      </PageContainer>
    );
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
                <div className="mb-2 flex flex-wrap items-center gap-2 text-xs text-muted-foreground">
                  <Badge variant="secondary">{solution.problemId}</Badge>
                  <span>{solution.problemTitle}</span>
                </div>
                <div className="text-xs text-muted-foreground">{dateTimeFormatter.format(new Date(solution.createdAt))}</div>
                </CardContent>
              </Card>
            </Link>
          ))}
        </div>
      )}
    </PageContainer>
  );
}
