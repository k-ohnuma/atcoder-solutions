import { Heart } from "lucide-react";
import Link from "next/link";
import { PageContainer } from "@/components/layout/PageContainer";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent } from "@/components/ui/card";
import { serverConfig } from "@/shared/config/backend";
import { SolutionListItem } from "@/shared/model/solution";

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

async function getLatestSolutions(): Promise<SolutionListItem[]> {
  const url = new URL("/api/solutions/latest", serverConfig.appConfig.appOrigin);
  url.searchParams.set("size", "50");

  const res = await fetch(url, {
    method: "GET",
    headers: {
      Accept: "application/json",
    },
  });

  const json = (await res.json()) as {
    ok?: boolean;
    data?: SolutionListItem[];
    error?: string;
  };
  if (!res.ok || !json.ok || !json.data) {
    throw new Error(`failed to fetch latest solutions: status=${res.status}, error=${json.error ?? "unknown error"}`);
  }

  return json.data;
}

export default async function RecentSolutionsPage() {
  const solutions = await getLatestSolutions();

  return (
    <PageContainer as="div">
      <div className="mb-6">
        <h1 className="text-2xl font-bold">最近の解説記事</h1>
      </div>

      {solutions.length === 0 ? (
        <p className="text-sm text-muted-foreground">解説はまだありません。</p>
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
                    <span>投稿者: {solution.userName}</span>
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
