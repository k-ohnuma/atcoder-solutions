import { useEffect, useRef, useState } from "react";
import { problemApi } from "@/features/problems/api/problemApi";
import { ContestProblemGroup, ContestProblemGroupPage, SupportedSeries } from "@/features/problems/model/contestProblemGroup";

type UseContestProblemGroupsParams = {
  selectedSeries: SupportedSeries;
  initialPage: ContestProblemGroupPage;
  pageSize: number;
};

function appendUniqueGroups(current: ContestProblemGroup[], incoming: ContestProblemGroup[]): ContestProblemGroup[] {
  const existingContestIds = new Set(current.map((group) => group.contestId));
  const next = [...current];
  for (const group of incoming) {
    if (existingContestIds.has(group.contestId)) {
      continue;
    }
    existingContestIds.add(group.contestId);
    next.push(group);
  }
  return next;
}

export function useContestProblemGroups({ selectedSeries, initialPage, pageSize }: UseContestProblemGroupsParams) {
  const [groups, setGroups] = useState<ContestProblemGroup[]>(initialPage.groups);
  const [canLoadMore, setCanLoadMore] = useState(initialPage.hasMore);
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const [loadMoreError, setLoadMoreError] = useState<string | null>(null);
  const requestVersionRef = useRef(0);
  const isLoadingMoreRef = useRef(false);

  useEffect(() => {
    requestVersionRef.current += 1;
    isLoadingMoreRef.current = false;
    setGroups(initialPage.groups);
    setCanLoadMore(initialPage.hasMore);
    setIsLoadingMore(false);
    setLoadMoreError(null);
  }, [initialPage]);

  const loadMore = async () => {
    if (isLoadingMoreRef.current) {
      return;
    }
    isLoadingMoreRef.current = true;
    const requestVersion = requestVersionRef.current;
    setIsLoadingMore(true);
    setLoadMoreError(null);

    try {
      const page = await problemApi.getContestProblemGroupPage({
        series: selectedSeries,
        limit: pageSize,
        offset: groups.length,
      });
      if (requestVersion !== requestVersionRef.current) {
        return;
      }

      setGroups((current) => appendUniqueGroups(current, page.groups));
      setCanLoadMore(page.hasMore);
    } catch {
      if (requestVersion === requestVersionRef.current) {
        setLoadMoreError("追加読み込みに失敗しました");
      }
    } finally {
      if (requestVersion === requestVersionRef.current) {
        isLoadingMoreRef.current = false;
        setIsLoadingMore(false);
      }
    }
  };

  return {
    groups,
    canLoadMore,
    isLoadingMore,
    loadMoreError,
    totalMatchedProblems: groups.reduce((acc, group) => acc + group.problems.length, 0),
    loadMore,
  };
}
