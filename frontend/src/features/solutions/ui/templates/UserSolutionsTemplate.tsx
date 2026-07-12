import { PageContainer } from "@/components/layout/PageContainer";
import { toUserSolutionSummaryListItem } from "@/features/solutions/model/solutionList";
import { SolutionListSortBy, UserSolutionListItem } from "@/shared/model/solution";
import { SolutionSortLinks } from "../molecules/SolutionSortLinks";
import { SolutionSummaryList } from "../organisms/SolutionSummaryList";

type UserSolutionsTemplateProps = {
  userName: string;
  selectedSort: SolutionListSortBy;
  solutions: UserSolutionListItem[];
};

export function UserSolutionsTemplate({ userName, selectedSort, solutions }: UserSolutionsTemplateProps) {
  const summarySolutions = solutions.map(toUserSolutionSummaryListItem);

  return (
    <PageContainer as="div">
      <h1 className="mb-4 text-2xl font-bold">@{userName} の解説一覧</h1>
      <SolutionSortLinks
        selectedSort={selectedSort}
        latestHref={`/users/${encodeURIComponent(userName)}/solutions?sortBy=latest`}
        votesHref={`/users/${encodeURIComponent(userName)}/solutions?sortBy=votes`}
      />

      <SolutionSummaryList solutions={summarySolutions} emptyMessage="このユーザーの解説はまだありません。" />
    </PageContainer>
  );
}
