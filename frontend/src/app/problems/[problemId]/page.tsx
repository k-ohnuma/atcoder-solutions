import Link from "next/link";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { ApiClient } from "@/lib/server/apiClient";
import { serverConfig } from "@/shared/config/backend";

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
};

export default async function ProblemSolutionsPage({ params }: PageProps) {
  const { problemId } = await params;
  const solutionsResp = await apiClient.getSolutionsByProblemId(problemId);

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

      {solutions.length === 0 ? (
        <p className="text-sm text-muted-foreground">この問題の解説はまだありません。</p>
      ) : (
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>タイトル</TableHead>
              <TableHead>投稿者</TableHead>
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
                <TableCell>{dateTimeFormatter.format(new Date(solution.createdAt))}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      )}
    </div>
  );
}
