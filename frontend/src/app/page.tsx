import { PageContainer } from "@/components/layout/PageContainer";
import { getContestProblemGroups } from "@/features/problems/api/getContestProblemGroups";
import { ContestProblemGroupPage, normalizeSeries } from "@/features/problems/model/contestProblemGroup";
import { ContestProblemGroupsContainer } from "@/features/problems/ui/containers/ContestProblemGroupsContainer";

const INITIAL_RENDER_LIMIT = 50;

type HomePageProps = {
  searchParams: Promise<{
    series?: string;
    q?: string;
  }>;
};

export default async function Home({ searchParams }: HomePageProps) {
  const { series, q } = await searchParams;
  const selectedSeries = normalizeSeries(series);
  const query = q?.trim() ?? "";
  let errorMessage: string | null = null;
  let initialPage: ContestProblemGroupPage = {
    groups: [],
    hasMore: false,
    totalContestCount: 0,
  };
  try {
    initialPage = await getContestProblemGroups({
      series: selectedSeries,
      query,
      ...(!query && { limit: INITIAL_RENDER_LIMIT, offset: 0 }),
    });
  } catch {
    errorMessage = query ? "検索に失敗しました" : "問題一覧の取得に失敗しました";
  }

  return (
    <PageContainer>
      <ContestProblemGroupsContainer
        key={`${selectedSeries}:${query}`}
        selectedSeries={selectedSeries}
        query={query}
        initialPage={initialPage}
        errorMessage={errorMessage}
        pageSize={INITIAL_RENDER_LIMIT}
      />
    </PageContainer>
  );
}
