import Link from "next/link";
import { PageContainer } from "@/components/layout/PageContainer";
import { Badge } from "@/components/ui/badge";
import { buildAtcoderProblemUrl } from "@/lib/atcoder";
import { SolutionComment } from "@/shared/model/solution";
import { MarkdownRenderer } from "../atoms/MarkdownRenderer";
import { LikeButton } from "../molecules/LikeButton";
import { SolutionMetadataAccordion } from "../molecules/SolutionMetadataAccordion";
import { SolutionComments } from "../organisms/SolutionComments";

type SolutionDetailTemplateProps = {
  solutionId: string;
  title: string;
  problemId: string;
  contestCode: string;
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

export function SolutionDetailTemplate({
  solutionId,
  title,
  problemId,
  contestCode,
  problemTitle,
  userId,
  userName,
  tags,
  bodyMd,
  submitUrl,
  createdAt,
  initialVotesCount,
  initialComments,
}: SolutionDetailTemplateProps) {
  const atcoderProblemUrl = buildAtcoderProblemUrl(problemId, contestCode);

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
          <SolutionMetadataAccordion
            solutionId={solutionId}
            problemId={problemId}
            ownerUserId={userId}
            title={title}
            bodyMd={bodyMd}
            submitUrl={submitUrl}
            tags={tags}
            userName={userName}
            createdAt={createdAt}
            atcoderProblemUrl={atcoderProblemUrl}
          />
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
