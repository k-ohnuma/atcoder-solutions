import { PageContainer } from "@/components/layout/PageContainer";
import { toSolutionSummaryListItem } from "@/features/solutions/model/solutionList";
import { SolutionListItem } from "@/shared/model/solution";
import { SolutionSummaryList } from "../organisms/SolutionSummaryList";

type RecentSolutionsTemplateProps = {
  solutions: SolutionListItem[];
};

export function RecentSolutionsTemplate({ solutions }: RecentSolutionsTemplateProps) {
  const summarySolutions = solutions.map(toSolutionSummaryListItem);

  return (
    <PageContainer as="div">
      <div className="mb-6">
        <h1 className="text-2xl font-bold">最近の解説記事</h1>
      </div>

      <SolutionSummaryList solutions={summarySolutions} emptyMessage="解説はまだありません。" />
    </PageContainer>
  );
}
