import { Heart } from "lucide-react";
import Link from "next/link";
import { PageContainer } from "@/components/layout/PageContainer";
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
      <h1 className="mb-1 text-2xl font-bold">{h1Title}</h1>
      <Link href={atcoderProblemUrl} target="_blank" rel="noreferrer" className="mb-4 inline-block text-sm hover:underline">
        AtCoder問題ページ
      </Link>
      <div className="mb-6 flex items-center gap-2 text-sm">
        <Link
          href={`/problems/${problemId}?sortBy=latest`}
          className={
            selectedSort === "latest"
              ? "rounded-md bg-primary px-3 py-1.5 font-medium text-primary-foreground"
              : "rounded-md border px-3 py-1.5 text-muted-foreground hover:bg-accent"
          }
        >
          新着順
        </Link>
        <Link
          href={`/problems/${problemId}?sortBy=votes`}
          className={
            selectedSort === "votes"
              ? "rounded-md bg-primary px-3 py-1.5 font-medium text-primary-foreground"
              : "rounded-md border px-3 py-1.5 text-muted-foreground hover:bg-accent"
          }
        >
          いいね順
        </Link>
      </div>

      {solutions.length === 0 ? (
        <p className="text-sm text-muted-foreground">この問題の解説はまだありません。</p>
      ) : (
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
          {solutions.map((solution) => (
            <article key={solution.id} className="rounded-xl border bg-card p-4">
              <div className="mb-3 flex items-start justify-between gap-3">
                <h2 className="line-clamp-2 text-lg font-semibold leading-snug">
                  <Link href={`/solutions/${solution.id}`} className="hover:underline">
                    {solution.title}
                  </Link>
                </h2>
                <div className="flex shrink-0 items-center gap-1 rounded-md border px-2 py-1 text-sm">
                  <Heart className="size-4" />
                  <span>{solution.votesCount}</span>
                </div>
              </div>

              <div className="mb-4 flex flex-wrap items-center gap-3 text-xs text-muted-foreground">
                <span>投稿者: {solution.userName}</span>
                <span>{dateTimeFormatter.format(new Date(solution.createdAt))}</span>
              </div>

              <Link href={`/solutions/${solution.id}`} className="text-sm font-medium hover:underline">
                解説を読む
              </Link>
            </article>
          ))}
        </div>
      )}
    </PageContainer>
  );
}
