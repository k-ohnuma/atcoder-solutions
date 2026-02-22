import Link from "next/link";
import { PageContainer } from "@/components/layout/PageContainer";
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from "@/components/ui/accordion";
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
    <PageContainer as="article">
      <div className="mx-auto w-full max-w-4xl space-y-5">
        <header className="rounded-xl border bg-card p-4">
          <div className="flex items-start justify-between gap-4">
            <div className="min-w-0 space-y-2">
              <div className="flex flex-wrap items-center gap-2">
                <Badge variant="secondary">{problemId}</Badge>
                <h1 className="text-lg font-semibold md:text-xl">{problemTitle} の解説</h1>
              </div>
              <p className="line-clamp-2 text-sm text-muted-foreground">
                {title} by{" "}
                <Link href={`/users/${encodeURIComponent(userName)}/solutions`} className="font-medium hover:underline">
                  @{userName}
                </Link>
              </p>
            </div>
            <LikeButton solutionId={solutionId} initialVotesCount={initialVotesCount} />
          </div>
          <Accordion type="single" collapsible className="mt-3">
            <AccordionItem value="meta" className="border-b-0">
              <AccordionTrigger className="justify-end py-2">
                <span className="sr-only">詳細情報</span>
              </AccordionTrigger>
              <AccordionContent className="pb-1">
                <Table>
                  <TableBody>
                    <TableRow>
                      <TableCell className="w-28 font-medium">投稿者</TableCell>
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
                      <TableCell className="font-medium">問題URL</TableCell>
                      <TableCell>
                        <Link
                          href={atcoderProblemUrl}
                          target="_blank"
                          rel="noreferrer"
                          className="text-sm break-all hover:underline"
                        >
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
                <div className="mt-3">
                  <SolutionOwnerActions
                    solutionId={solutionId}
                    problemId={problemId}
                    ownerUserId={userId}
                    title={title}
                    bodyMd={bodyMd}
                    submitUrl={submitUrl}
                    tags={tags}
                  />
                </div>
              </AccordionContent>
            </AccordionItem>
          </Accordion>
        </header>

        <section className="rounded-xl border bg-card">
          <div className="border-b px-4 py-3">
            <h2 className="text-lg font-semibold">解説</h2>
          </div>
          <div className="px-1 py-3 md:px-4">
            <MarkdownRenderer value={bodyMd} />
          </div>
        </section>

        <SolutionComments solutionId={solutionId} initialComments={initialComments} />
      </div>
    </PageContainer>
  );
}
