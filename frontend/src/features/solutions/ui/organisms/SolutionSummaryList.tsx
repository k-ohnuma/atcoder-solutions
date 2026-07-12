import { SolutionSummaryListItem } from "@/features/solutions/model/solutionList";
import { SolutionSummaryCard } from "../molecules/SolutionSummaryCard";

type SolutionSummaryListProps = {
  solutions: SolutionSummaryListItem[];
  emptyMessage: string;
  footerLabel?: string;
};

export function SolutionSummaryList({ solutions, emptyMessage, footerLabel }: SolutionSummaryListProps) {
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
          problemId={solution.problemId}
          problemTitle={solution.problemTitle}
          userName={solution.userName}
          footerLabel={footerLabel}
        />
      ))}
    </div>
  );
}
