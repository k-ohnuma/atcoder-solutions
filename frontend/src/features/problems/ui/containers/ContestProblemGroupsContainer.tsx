"use client";

import { useContestProblemGroups } from "@/features/problems/hooks/useContestProblemGroups";
import { ContestProblemGroupPage, SupportedSeries } from "@/features/problems/model/contestProblemGroup";
import { ProblemsTemplate } from "../templates/ProblemsTemplate";

type ContestProblemGroupsContainerProps = {
  selectedSeries: SupportedSeries;
  query: string;
  initialPage: ContestProblemGroupPage;
  errorMessage: string | null;
  pageSize: number;
};

export function ContestProblemGroupsContainer({
  selectedSeries,
  query,
  initialPage,
  errorMessage,
  pageSize,
}: ContestProblemGroupsContainerProps) {
  const { groups, canLoadMore, isLoadingMore, totalMatchedProblems, loadMore } = useContestProblemGroups({
    selectedSeries,
    initialPage,
    pageSize,
  });

  return (
    <ProblemsTemplate
      selectedSeries={selectedSeries}
      query={query}
      totalMatchedProblems={totalMatchedProblems}
      errorMessage={errorMessage}
      groups={groups}
      canLoadMore={canLoadMore}
      isLoadingMore={isLoadingMore}
      onLoadMore={loadMore}
    />
  );
}
