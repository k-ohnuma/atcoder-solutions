import { PageContainer } from "@/components/layout/PageContainer";
import { ProblemsTemplate } from "@/features/problems/ui/templates/ProblemsTemplate";
import { serverConfig } from "@/shared/config/backend";
import { Problem } from "@/shared/model/problem";

const supportedSeries = ["ABC", "ARC", "AGC", "AHC", "OTHER"] as const;
type SupportedSeries = (typeof supportedSeries)[number];
type ContestGroupCollection = Map<string, Problem[]>;

const INITIAL_RENDER_LIMIT = 50;

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

async function getContestGroupBySeries(series: SupportedSeries): Promise<ContestGroupCollection> {
  const url = new URL("/api/problems/contest-group", serverConfig.appConfig.appOrigin);
  url.searchParams.set("series", series);

  const res = await fetch(url, {
    method: "GET",
    headers: {
      Accept: "application/json",
    },
  });
  const json = (await res.json()) as {
    ok?: boolean;
    data?: Record<string, Problem[]>;
    error?: string;
  };
  if (!res.ok || !json.ok || !json.data) {
    throw new Error(`failed to fetch contest groups: status=${res.status}, error=${json.error ?? "unknown error"}`);
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

  const shouldShowAll = showAll === "1" || normalizedQuery.length > 0;
  const visibleList = shouldShowAll ? list : list.slice(0, INITIAL_RENDER_LIMIT);
  const remainingContestCount = Math.max(0, list.length - visibleList.length);
  const totalMatchedProblems = visibleList.reduce((acc, [, problems]) => acc + problems.length, 0);

  return (
    <PageContainer>
      <ProblemsTemplate
        selectedSeries={selectedSeries}
        query={query}
        totalMatchedProblems={totalMatchedProblems}
        visibleContests={visibleList}
        shouldShowAll={shouldShowAll}
        remainingContestCount={remainingContestCount}
      />
    </PageContainer>
  );
}
