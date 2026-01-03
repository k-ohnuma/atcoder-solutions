import { Problem } from "@/shared/model/problem";
import { ContestAccordion } from "../organisis/ContestAccordion";

export const ProblemsTemplate = ({ problemsMap }: { problemsMap: Map<string, Problem[]> }) => {
  return (
    <div className="mx-auto max-w-7xl">
      <ContestAccordion problemsMap={problemsMap} />
    </div>
  );
};
