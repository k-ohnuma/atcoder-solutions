import { SolutionListItem, SolutionListSortBy } from "@/shared/model/solution";
import { ButtonLink } from "@/shared/ui/ButtonLink";
import { SolutionSummaryCard } from "../molecules";

type ProblemSolutionListProps = {
  problemId: string;
  selectedSort: SolutionListSortBy;
  solutions: SolutionListItem[];
};

export function ProblemSolutionList({ problemId, selectedSort, solutions }: ProblemSolutionListProps) {
  return (
    <>
      <div className="mb-6 flex items-center gap-2 text-sm">
        <ButtonLink
          href={`/problems/${problemId}?sortBy=latest`}
          size="sm"
          variant={selectedSort === "latest" ? "default" : "outline"}
        >
          新着順
        </ButtonLink>
        <ButtonLink
          href={`/problems/${problemId}?sortBy=votes`}
          size="sm"
          variant={selectedSort === "votes" ? "default" : "outline"}
        >
          いいね順
        </ButtonLink>
      </div>

      {solutions.length === 0 ? (
        <p className="text-sm text-muted-foreground">この問題の解説はまだありません。</p>
      ) : (
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
          {solutions.map((solution) => (
            <SolutionSummaryCard
              key={solution.id}
              href={`/solutions/${solution.id}`}
              title={solution.title}
              votesCount={solution.votesCount}
              createdAt={solution.createdAt}
              userName={solution.userName}
              footerLabel="解説を読む"
            />
          ))}
        </div>
      )}
    </>
  );
}
