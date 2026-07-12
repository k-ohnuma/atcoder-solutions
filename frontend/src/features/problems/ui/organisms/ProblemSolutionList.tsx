import { SolutionSortLinks } from "@/features/solutions/ui/molecules/SolutionSortLinks";
import { SolutionSummaryCard } from "@/features/solutions/ui/molecules/SolutionSummaryCard";
import { SolutionListItem, SolutionListSortBy } from "@/shared/model/solution";

type ProblemSolutionListProps = {
  problemId: string;
  selectedSort: SolutionListSortBy;
  solutions: SolutionListItem[];
};

export function ProblemSolutionList({ problemId, selectedSort, solutions }: ProblemSolutionListProps) {
  return (
    <>
      <SolutionSortLinks
        selectedSort={selectedSort}
        latestHref={`/problems/${problemId}?sortBy=latest`}
        votesHref={`/problems/${problemId}?sortBy=votes`}
      />

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
