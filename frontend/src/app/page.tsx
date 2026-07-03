import { PageContainer } from "@/components/layout/PageContainer";
import { ProblemsTemplate } from "@/features/problems/ui/templates/ProblemsTemplate";
import { serverConfig } from "@/shared/config/backend";
import { Problem } from "@/shared/model/problem";

const supportedSeries = ["ABC", "ARC", "AGC", "AHC", "AWC", "OTHER"] as const;
type SupportedSeries = (typeof supportedSeries)[number];
type ContestGroupCollection = Map<string, Problem[]>;
type ContestGroupPage = {
  items: ContestGroupCollection;
  hasMore: boolean;
  totalContestCount: number;
};

const INITIAL_RENDER_LIMIT = 50;

type HomePageProps = {
  searchParams: Promise<{
    series?: string;
    q?: string;
  }>;
};

function normalizeSeries(value?: string): SupportedSeries {
  if (value && supportedSeries.includes(value as SupportedSeries)) {
    return value as SupportedSeries;
  }
  return "ABC";
}

async function getContestGroupBySeries(series: SupportedSeries, query: string): Promise<ContestGroupPage> {
  const url = new URL("/api/problems/contest-group", serverConfig.appConfig.appOrigin);
  url.searchParams.set("series", series);
  if (query) {
    url.searchParams.set("q", query);
  } else {
    url.searchParams.set("limit", String(INITIAL_RENDER_LIMIT));
    url.searchParams.set("offset", "0");
  }

  const res = await fetch(url, {
    method: "GET",
    headers: {
      Accept: "application/json",
    },
  });
  const json = (await res.json()) as {
    ok?: boolean;
    data?: {
      items: Record<string, Problem[]>;
      hasMore: boolean;
      totalContestCount: number;
    };
    error?: string;
  };
  if (!res.ok || !json.ok || !json.data) {
    throw new Error(`failed to fetch contest groups: status=${res.status}, error=${json.error ?? "unknown error"}`);
  }

  return {
    ...json.data,
    items: new Map<string, Problem[]>(Object.entries(json.data.items)),
  };
}

export default async function Home({ searchParams }: HomePageProps) {
  const { series, q } = await searchParams;
  const selectedSeries = normalizeSeries(series);
  const query = q?.trim() ?? "";
  const contestGroupPage = await getContestGroupBySeries(selectedSeries, query);
  const visibleList = [...contestGroupPage.items.entries()];
  const totalMatchedProblems = visibleList.reduce((acc, [, problems]) => acc + problems.length, 0);

  return (
    <PageContainer>
      <ProblemsTemplate
        key={`${selectedSeries}:${query}`}
        selectedSeries={selectedSeries}
        query={query}
        totalMatchedProblems={totalMatchedProblems}
        visibleContests={visibleList}
        hasMore={contestGroupPage.hasMore}
        pageSize={INITIAL_RENDER_LIMIT}
      />
    </PageContainer>
  );
}
