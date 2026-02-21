import Link from "next/link";
import { PageContainer } from "@/components/layout/PageContainer";
import { ApiClient } from "@/lib/server/apiClient";
import { serverConfig } from "@/shared/config/backend";

const apiClient = new ApiClient(serverConfig.appConfig.apiBaseEndpoint);

const supportedSeries = ["ABC", "ARC", "AGC", "AHC", "OTHER"] as const;
type SupportedSeries = (typeof supportedSeries)[number];

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

export default async function Home({ searchParams }: HomePageProps) {
  const { series } = await searchParams;
  const selectedSeries = normalizeSeries(series);
  const contestGroupCollection = await apiClient.getContestGroupByContestSeries(selectedSeries);
  const list = [...contestGroupCollection.entries()].sort(([a], [b]) => b.localeCompare(a));

  return (
    <PageContainer>
      <section className="mb-6 flex flex-col gap-3">
        <div className="flex flex-wrap gap-2">
          {supportedSeries.map((code) => {
            const active = code === selectedSeries;
            return (
              <Link
                key={code}
                href={`/?series=${code}`}
                className={
                  active
                    ? "inline-flex h-8 items-center rounded-md bg-primary px-3 text-sm font-medium text-primary-foreground"
                    : "inline-flex h-8 items-center rounded-md border bg-background px-3 text-sm font-medium hover:bg-accent"
                }
              >
                {code}
              </Link>
            );
          })}
        </div>
      </section>

      <section className="space-y-3">
        {list.map(([contestId, problems]) => (
          <article key={contestId} className="rounded-xl border bg-background p-4">
            <h2 className="mb-3 text-base font-semibold">{contestId}</h2>
            <div className="grid gap-2 sm:grid-cols-2 lg:grid-cols-3">
              {[...problems]
                .sort((a, b) => a.problemIndex.localeCompare(b.problemIndex))
                .map((problem) => (
                  <Link
                    key={problem.id}
                    href={`/problems/${problem.id}`}
                    className="block rounded-md border px-3 py-2 transition-colors hover:bg-accent"
                  >
                    <p className="mb-1 text-xs text-muted-foreground">{problem.problemIndex}</p>
                    <p className="text-sm font-medium">{problem.title}</p>
                  </Link>
                ))}
            </div>
          </article>
        ))}
      </section>
    </PageContainer>
  );
}
