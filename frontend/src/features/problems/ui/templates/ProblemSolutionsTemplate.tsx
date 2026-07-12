import { PageContainer } from "@/components/layout/PageContainer";
import { Problem } from "@/shared/model/problem";
import { SolutionListItem, SolutionListSortBy } from "@/shared/model/solution";
import { ProblemSolutionList } from "../organisms/ProblemSolutionList";
import { ProblemSolutionsHeader } from "../organisms/ProblemSolutionsHeader";

type ProblemSolutionsTemplateProps = {
  problem: Problem;
  selectedSort: SolutionListSortBy;
  solutions: SolutionListItem[];
};

export function ProblemSolutionsTemplate({ problem, selectedSort, solutions }: ProblemSolutionsTemplateProps) {
  return (
    <PageContainer as="div">
      <ProblemSolutionsHeader problem={problem} />
      <ProblemSolutionList problemId={problem.id} selectedSort={selectedSort} solutions={solutions} />
    </PageContainer>
  );
}
