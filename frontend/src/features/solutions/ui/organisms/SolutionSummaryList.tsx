import { SolutionListItem, UserSolutionListItem } from "@/shared/model/solution";
import { SolutionSummaryCard } from "../molecules/SolutionSummaryCard";

type SummarySolution = SolutionListItem | UserSolutionListItem;

type SolutionSummaryListProps = {
  solutions: SummarySolution[];
  emptyMessage: string;
  showUserName?: boolean;
  showProblem?: boolean;
  footerLabel?: string;
};

export function SolutionSummaryList({
  solutions,
  emptyMessage,
  showUserName = false,
  showProblem = false,
  footerLabel,
}: SolutionSummaryListProps) {
  if (solutions.length === 0) {
    return <p className="text-sm text-muted-foreground">{emptyMessage}</p>;
  }

  return (
    <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
      {solutions.map((solution) => (
        <SolutionSummaryCard
          key={solution.id}
          href={`/solutions/${solution.id}`}
          title={solution.title}
          votesCount={solution.votesCount}
          createdAt={solution.createdAt}
          problemId={showProblem ? solution.problemId : undefined}
          problemTitle={showProblem && "problemTitle" in solution ? solution.problemTitle : undefined}
          userName={showUserName && "userName" in solution ? solution.userName : undefined}
          footerLabel={footerLabel}
        />
      ))}
    </div>
  );
}
