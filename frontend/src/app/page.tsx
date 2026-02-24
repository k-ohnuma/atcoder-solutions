import Link from "next/link";
import { PageContainer } from "@/components/layout/PageContainer";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { serverConfig } from "@/shared/config/backend";
import { Problem } from "@/shared/model/problem";

const supportedSeries = ["ABC", "ARC", "AGC", "AHC", "OTHER"] as const;
type SupportedSeries = (typeof supportedSeries)[number];
type ContestGroupCollection = Map<string, Problem[]>;

type HomePageProps = {
  searchParams: Promise<{
    series?: string;
  }>;
};

function normalizeSeries(value?: string): SupportedSeries {
  if (value && supportedSeries.includes(value as SupportedSeries)) {
    return value as SupportedSeries;
  }
  return "ABC";
}

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
  const { series } = await searchParams;
  const selectedSeries = normalizeSeries(series);
  const contestGroupCollection = await getContestGroupBySeries(selectedSeries);
  const list = [...contestGroupCollection.entries()].sort(([a], [b]) => b.localeCompare(a));

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
      </section>

      <section className="space-y-3">
        {list.map(([contestId, problems]) => (
          <Card key={contestId}>
            <CardHeader className="pb-3">
              <CardTitle className="text-base">{contestId}</CardTitle>
            </CardHeader>
            <CardContent className="grid gap-2 pt-0 sm:grid-cols-2 lg:grid-cols-3">
              {[...problems]
                .sort((a, b) => a.problemIndex.localeCompare(b.problemIndex))
                .map((problem) => (
                  <Link
                    key={problem.id}
                    href={`/problems/${problem.id}`}
                    className="block rounded-md border bg-background px-3 py-2 transition-colors hover:bg-accent"
                  >
                    <Badge variant="outline" className="mb-1">
                      {problem.problemIndex}
                    </Badge>
                    <p className="text-sm font-medium">{problem.title}</p>
                  </Link>
                ))}
            </CardContent>
          </Card>
        ))}
      </section>
    </PageContainer>
  );
}
