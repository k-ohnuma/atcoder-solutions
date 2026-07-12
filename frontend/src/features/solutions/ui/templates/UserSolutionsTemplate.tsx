import { PageContainer } from "@/components/layout/PageContainer";
import { SolutionListSortBy, UserSolutionListItem } from "@/shared/model/solution";
import { SolutionSortLinks } from "../molecules/SolutionSortLinks";
import { SolutionSummaryList } from "../organisms/SolutionSummaryList";

type UserSolutionsTemplateProps = {
  userName: string;
  selectedSort: SolutionListSortBy;
  solutions: UserSolutionListItem[];
};

export function UserSolutionsTemplate({ userName, selectedSort, solutions }: UserSolutionsTemplateProps) {
  return (
    <PageContainer as="div">
      <h1 className="mb-4 text-2xl font-bold">@{userName} の解説一覧</h1>
      <SolutionSortLinks
        selectedSort={selectedSort}
        latestHref={`/users/${encodeURIComponent(userName)}/solutions?sortBy=latest`}
        votesHref={`/users/${encodeURIComponent(userName)}/solutions?sortBy=votes`}
      />

      <SolutionSummaryList solutions={solutions} emptyMessage="このユーザーの解説はまだありません。" showProblem />
    </PageContainer>
  );
}
