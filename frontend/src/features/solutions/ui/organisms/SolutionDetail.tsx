import Link from "next/link";
import { PageContainer } from "@/components/layout/PageContainer";
import { Badge } from "@/components/ui/badge";
import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import { buildAtcoderProblemUrl } from "@/lib/atcoder";
import { SolutionComment } from "@/shared/model/solution";
import { MarkdownRenderer } from "../atoms";
import { LikeButton } from "../molecules/LikeButton";
import { SolutionOwnerActions } from "../molecules/SolutionOwnerActions";
import { SolutionComments } from "./SolutionComments";

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

type SolutionDetailProps = {
  solutionId: string;
  title: string;
  problemId: string;
  problemTitle: string;
  userId: string;
  userName: string;
  tags: string[];
  bodyMd: string;
  submitUrl: string;
  createdAt: string;
  initialVotesCount: number;
  initialComments: SolutionComment[];
};

export function SolutionDetail({
  solutionId,
  title,
  problemId,
  problemTitle,
  userId,
  userName,
  tags,
  bodyMd,
  submitUrl,
  createdAt,
  initialVotesCount,
  initialComments,
}: SolutionDetailProps) {
  const atcoderProblemUrl = buildAtcoderProblemUrl(problemId);

  return (
    <PageContainer as="article" className="space-y-6">
      <header className="space-y-3  pb-4">
        <div className="flex items-start justify-between gap-4">
          <div className="space-y-1">
            <h1 className="text-xl font-semibold md:text-2xl">
              {problemId} / {problemTitle} の解説
            </h1>
            <p className="text-sm text-muted-foreground">{title}</p>
          </div>
          <LikeButton solutionId={solutionId} initialVotesCount={initialVotesCount} />
        </div>
        <SolutionOwnerActions
          solutionId={solutionId}
          problemId={problemId}
          ownerUserId={userId}
          title={title}
          bodyMd={bodyMd}
          submitUrl={submitUrl}
          tags={tags}
        />
        <Table>
          <TableBody>
            <TableRow>
              <TableCell className="w-36 font-medium">投稿者</TableCell>
              <TableCell>
                <Link href={`/users/${encodeURIComponent(userName)}/solutions`} className="hover:underline">
                  {userName}
                </Link>
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell className="font-medium">投稿日</TableCell>
              <TableCell>{dateTimeFormatter.format(new Date(createdAt))}</TableCell>
            </TableRow>
            <TableRow>
              <TableCell className="font-medium">問題</TableCell>
              <TableCell>
                <Link href={atcoderProblemUrl} target="_blank" rel="noreferrer" className="text-sm break-all hover:underline">
                  {atcoderProblemUrl}
                </Link>
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell className="font-medium">提出URL</TableCell>
              <TableCell>
                {submitUrl ? (
                  <Link href={submitUrl} target="_blank" rel="noreferrer" className="text-sm break-all hover:underline">
                    {submitUrl}
                  </Link>
                ) : (
                  <span className="text-sm text-muted-foreground">なし</span>
                )}
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell className="font-medium">タグ</TableCell>
              <TableCell>
                <div className="flex flex-wrap gap-2">
                  {tags.length === 0 ? (
                    <Badge variant="outline">タグなし</Badge>
                  ) : (
                    tags.map((tag) => (
                      <Badge key={tag} variant="outline">
                        {tag}
                      </Badge>
                    ))
                  )}
                </div>
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </header>

      <section className="rounded-xl border bg-card">
        <div className="border-b px-4 py-3">
          <h2 className="text-lg font-semibold">解説</h2>
        </div>
        <MarkdownRenderer value={bodyMd} />
      </section>

      <SolutionComments solutionId={solutionId} initialComments={initialComments} />
    </PageContainer>
  );
}
