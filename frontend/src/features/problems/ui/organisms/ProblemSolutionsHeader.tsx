import { ExternalLink } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { buildAtcoderProblemUrl } from "@/lib/atcoder";
import { Problem } from "@/shared/model/problem";
import { ButtonLink } from "@/shared/ui/ButtonLink";
import { DifficultyBadge } from "../atoms/DifficultyBadge";

type ProblemSolutionsHeaderProps = {
  problem: Problem;
};

export function ProblemSolutionsHeader({ problem }: ProblemSolutionsHeaderProps) {
  const atcoderProblemUrl = buildAtcoderProblemUrl(problem.id, problem.contestCode);
  const difficulty = problem.difficulty ?? null;

  return (
    <div className="mb-4 flex flex-wrap items-center justify-between gap-2">
      <div className="min-w-0 space-y-2">
        <div className="flex flex-wrap items-center gap-2">
          <Badge variant="secondary">{problem.id}</Badge>
          <h1 className="text-lg font-semibold md:text-xl">{problem.title} の解説一覧</h1>
          <DifficultyBadge problemIndex={problem.problemIndex} difficulty={difficulty} label={difficulty ?? "N/A"} />
        </div>
      </div>
      <div className="flex flex-wrap items-center gap-2">
        <ButtonLink href={atcoderProblemUrl} target="_blank" rel="noreferrer" variant="outline" size="sm">
          <ExternalLink className="size-4" />
          AtCoder問題ページを開く
        </ButtonLink>
        <ButtonLink href={`/solutions/create?problemId=${encodeURIComponent(problem.id)}`} size="sm">
          この問題の記事を書く
        </ButtonLink>
      </div>
    </div>
  );
}
