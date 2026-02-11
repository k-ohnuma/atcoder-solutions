import Link from "next/link";
import { MarkdownRenderer } from "../atoms";

type SolutionDetailProps = {
  title: string;
  problemId: string;
  problemTitle: string;
  userName: string;
  bodyMd: string;
  submitUrl: string;
  createdAt: string;
};

export function SolutionDetail({
  title,
  problemId,
  problemTitle,
  userName,
  bodyMd,
  submitUrl,
  createdAt,
}: SolutionDetailProps) {
  return (
    <article className="mx-auto w-full max-w-4xl space-y-6 p-4 md:p-8">
      <header className="space-y-3 border-b pb-4">
        <h1 className="text-2xl font-bold md:text-3xl">{title}</h1>
        <div className="flex flex-wrap items-center gap-3 text-sm text-muted-foreground">
          <span>by {userName}</span>
          <span>{new Date(createdAt).toLocaleString("ja-JP")}</span>
          <span>
            {problemId}: {problemTitle}
          </span>
        </div>
        {submitUrl && (
          <Link href={submitUrl} target="_blank" rel="noreferrer" className="inline-block text-sm hover:underline">
            提出URL
          </Link>
        )}
      </header>

      <section className="rounded-xl border bg-card">
        <MarkdownRenderer value={bodyMd} />
      </section>
    </article>
  );
}
