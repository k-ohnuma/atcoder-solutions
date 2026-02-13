import Link from "next/link";
import { SolutionComment } from "@/shared/model/solution";
import { MarkdownRenderer } from "../atoms";
import { LikeButton } from "../molecules/LikeButton";
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
  userName: string;
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
  userName,
  bodyMd,
  submitUrl,
  createdAt,
  initialVotesCount,
  initialComments,
}: SolutionDetailProps) {
  return (
    <article className="mx-auto w-full max-w-4xl space-y-6 p-4 md:p-8">
      <header className="space-y-3 border-b pb-4">
        <h1 className="text-2xl font-bold md:text-3xl">{title}</h1>
        <div className="flex flex-wrap items-center gap-3 text-sm text-muted-foreground">
          <span>by {userName}</span>
          <span>{dateTimeFormatter.format(new Date(createdAt))}</span>
          <span>
            {problemId}: {problemTitle}
          </span>
        </div>
        {submitUrl && (
          <Link href={submitUrl} target="_blank" rel="noreferrer" className="inline-block text-sm hover:underline">
            提出URL
          </Link>
        )}
        <LikeButton solutionId={solutionId} initialVotesCount={initialVotesCount} />
      </header>

      <section className="rounded-xl border bg-card">
        <MarkdownRenderer value={bodyMd} />
      </section>

      <SolutionComments solutionId={solutionId} initialComments={initialComments} />
    </article>
  );
}
