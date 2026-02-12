import Link from "next/link";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
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
  const selectedSort: SolutionListSortBy = sortBy === "votes" ? "votes" : "latest";
  const solutionsResp = await apiClient.getSolutionsByProblemId(problemId, selectedSort);

  if (!solutionsResp.ok) {
    return (
      <div className="mx-auto w-full max-w-4xl p-4 md:p-8">
        <h1 className="mb-3 text-2xl font-bold">解説一覧</h1>
        <p className="text-sm text-destructive">
          解説一覧の取得に失敗しました。status: {solutionsResp.status}, error: {solutionsResp.error}
        </p>
      </div>
    );
  }

  const solutions = solutionsResp.data;

  return (
    <div className="mx-auto w-full max-w-4xl p-4 md:p-8">
      <h1 className="mb-1 text-2xl font-bold">解説一覧</h1>
      <p className="mb-4 text-sm text-muted-foreground">problemId: {problemId}</p>
      <div className="mb-4 flex items-center gap-4 text-sm">
        <Link
          href={`/problems/${problemId}?sortBy=latest`}
          className={selectedSort === "latest" ? "font-semibold underline" : "text-muted-foreground hover:underline"}
        >
          新着順
        </Link>
        <Link
          href={`/problems/${problemId}?sortBy=votes`}
          className={selectedSort === "votes" ? "font-semibold underline" : "text-muted-foreground hover:underline"}
        >
          いいね順
        </Link>
      </div>

      {solutions.length === 0 ? (
        <p className="text-sm text-muted-foreground">この問題の解説はまだありません。</p>
      ) : (
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>タイトル</TableHead>
              <TableHead>投稿者</TableHead>
              <TableHead>いいね数</TableHead>
              <TableHead>投稿日</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {solutions.map((solution) => (
              <TableRow key={solution.id}>
                <TableCell>
                  <Link href={`/solutions/${solution.id}`} className="hover:underline">
                    {solution.title}
                  </Link>
                </TableCell>
                <TableCell>{solution.userName}</TableCell>
                <TableCell>{solution.votesCount}</TableCell>
                <TableCell>{dateTimeFormatter.format(new Date(solution.createdAt))}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      )}
    </div>
  );
}
