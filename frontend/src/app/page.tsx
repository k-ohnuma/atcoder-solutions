import Link from "next/link";
import { PageContainer } from "@/components/layout/PageContainer";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { ShowAllButton } from "@/features/problems/ui/molecules/ShowAllButton";
import { serverConfig } from "@/shared/config/backend";
import { Problem } from "@/shared/model/problem";

const supportedSeries = ["ABC", "ARC", "AGC", "AHC", "OTHER"] as const;
type SupportedSeries = (typeof supportedSeries)[number];
type ContestGroupCollection = Map<string, Problem[]>;

type HomePageProps = {
  searchParams: Promise<{
    series?: string;
    showAll?: string;
    q?: string;
  }>;
};

function normalizeSeries(value?: string): SupportedSeries {
  if (value && supportedSeries.includes(value as SupportedSeries)) {
    return value as SupportedSeries;
  }
  return "ABC";
}

function difficultyBadgeClass(difficulty?: number | null): string {
  if (difficulty === null || difficulty === undefined) {
    return "";
  }
  if (difficulty < 400) {
    return "border-transparent bg-[#808080] text-white";
  }
  if (difficulty < 800) {
    return "border-transparent bg-[#804000] text-white";
  }
  if (difficulty < 1200) {
    return "border-transparent bg-[#008000] text-white";
  }
  if (difficulty < 1600) {
    return "border-transparent bg-[#00C0C0] text-white";
  }
  if (difficulty < 2000) {
    return "border-transparent bg-[#0000FF] text-white";
  }
  if (difficulty < 2400) {
    return "border-transparent bg-[#C0C000] text-white";
  }
  if (difficulty < 2800) {
    return "border-transparent bg-[#FF8000] text-white";
  }
  return "border-transparent bg-[#FF0000] text-white";
}

const compactBadgeClass = "mb-1 px-1.5 py-0 text-[12px] leading-tight";

async function getContestGroupBySeries(series: SupportedSeries): Promise<ContestGroupCollection> {
  const url = new URL("/api/problems/contest-group", serverConfig.appConfig.appOrigin);
  url.searchParams.set("series", series);

  const res = await fetch(url, {
    method: "GET",
    headers: {
      Accept: "application/json",
    },
  });
  const json = (await res.json()) as { ok?: boolean; data?: Record<string, Problem[]> };
  if (!res.ok || !json.ok || !json.data) {
    return new Map();
  }

  return new Map<string, Problem[]>(Object.entries(json.data));
}

export default async function Home({ searchParams }: HomePageProps) {
  const { series, showAll, q } = await searchParams;
  const selectedSeries = normalizeSeries(series);
  const contestGroupCollection = await getContestGroupBySeries(selectedSeries);
  const query = q?.trim() ?? "";
  const normalizedQuery = query.toLowerCase();
  const list = [...contestGroupCollection.entries()]
    .map(([contestId, problems]) => {
      if (!normalizedQuery) {
        return [contestId, problems] as const;
      }
      const filteredProblems = problems.filter((problem) => {
        const problemTitle = problem.title.toLowerCase();
        const problemIndex = problem.problemIndex.toLowerCase();
        const problemId = problem.id.toLowerCase();
        const contestCode = contestId.toLowerCase();
        return (
          problemTitle.includes(normalizedQuery) ||
          problemIndex.includes(normalizedQuery) ||
          problemId.includes(normalizedQuery) ||
          contestCode.includes(normalizedQuery)
        );
      });
      return [contestId, filteredProblems] as const;
    })
    .filter(([, problems]) => problems.length > 0);

  const INITIAL_RENDER_LIMIT = 50;
  const shouldShowAll = showAll === "1" || normalizedQuery.length > 0;
  const visibleList = shouldShowAll ? list : list.slice(0, INITIAL_RENDER_LIMIT);
  const remainingContestCount = Math.max(0, list.length - visibleList.length);
  const totalMatchedProblems = visibleList.reduce((acc, [, problems]) => acc + problems.length, 0);

  return (
    <PageContainer>
      <section className="mb-6 flex flex-col gap-3">
        <div className="flex flex-wrap gap-2">
          {supportedSeries.map((code) => {
            const active = code === selectedSeries;
            return (
              <Button key={code} asChild size="sm" variant={active ? "default" : "outline"}>
                <Link href={`/?series=${code}`}>{code}</Link>
              </Button>
            );
          })}
        </div>

        <form method="GET" className="flex w-full items-center gap-2">
          <input type="hidden" name="series" value={selectedSeries} />
          <Input name="q" placeholder="検索" defaultValue={query} />
          <Button type="submit" variant="outline">
            検索
          </Button>
          {query && (
            <Button asChild type="button" variant="ghost">
              <Link href={`/?series=${selectedSeries}`}>クリア</Link>
            </Button>
          )}
        </form>
        {query && (
          <p className="text-sm text-muted-foreground">
            「{query}」の検索: {totalMatchedProblems} 件
          </p>
        )}
      </section>

      <section className="space-y-3">
        {visibleList.map(([contestId, problems]) => (
          <Card key={contestId}>
            <CardHeader className="pb-3">
              <CardTitle className="text-base">{contestId}</CardTitle>
            </CardHeader>
            <CardContent className="grid gap-2 pt-0 sm:grid-cols-2 lg:grid-cols-3">
              {problems.map((problem) => (
                <Link
                  key={problem.id}
                  href={`/problems/${problem.id}`}
                  className="block rounded-md border bg-background px-3 py-2 transition-colors hover:bg-accent"
                >
                  <Badge
                    variant="outline"
                    className={`${compactBadgeClass} ${difficultyBadgeClass(problem.difficulty)}`}
                    title={`Difficulty: ${problem.difficulty ?? "N/A"}`}
                  >
                    {problem.problemIndex}
                  </Badge>
                  <p className="text-sm font-medium">{problem.title}</p>
                </Link>
              ))}
            </CardContent>
          </Card>
        ))}
      </section>

      {!shouldShowAll && remainingContestCount > 0 && (
        <section className="mt-6 flex justify-center">
          <ShowAllButton href={`/?series=${selectedSeries}&showAll=1`} remainingContestCount={remainingContestCount} />
        </section>
      )}
    </PageContainer>
  );
}
