"use client";

import { useEffect, useRef, useState } from "react";
import { fetchMoreContestProblemGroups } from "@/features/problems/api/fetchMoreContestProblemGroups";
import { ContestProblemGroup, ContestProblemGroupPage, SupportedSeries } from "@/features/problems/model/contestProblemGroup";
import { ProblemsTemplate } from "../templates/ProblemsTemplate";

type ContestProblemGroupsContainerProps = {
  selectedSeries: SupportedSeries;
  query: string;
  initialPage: ContestProblemGroupPage;
  errorMessage: string | null;
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

export function ContestProblemGroupsContainer({
  selectedSeries,
  query,
  initialPage,
  errorMessage,
  pageSize,
}: ContestProblemGroupsContainerProps) {
  const [groups, setGroups] = useState<ContestProblemGroup[]>(initialPage.groups);
  const [canLoadMore, setCanLoadMore] = useState(initialPage.hasMore);
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const requestVersionRef = useRef(0);
  const isLoadingMoreRef = useRef(false);

  useEffect(() => {
    requestVersionRef.current += 1;
    isLoadingMoreRef.current = false;
    setGroups(initialPage.groups);
    setCanLoadMore(initialPage.hasMore);
    setIsLoadingMore(false);
  }, [initialPage]);

  const loadMore = async () => {
    if (isLoadingMoreRef.current) {
      return;
    }
    isLoadingMoreRef.current = true;
    const requestVersion = requestVersionRef.current;
    setIsLoadingMore(true);

    try {
      const page = await fetchMoreContestProblemGroups({
        series: selectedSeries,
        limit: pageSize,
        offset: groups.length,
      });
      if (requestVersion !== requestVersionRef.current) {
        return;
      }

      setGroups((current) => appendUniqueGroups(current, page.groups));
      setCanLoadMore(page.hasMore);
    } finally {
      if (requestVersion === requestVersionRef.current) {
        isLoadingMoreRef.current = false;
        setIsLoadingMore(false);
      }
    }
  };

  const totalMatchedProblems = groups.reduce((acc, group) => acc + group.problems.length, 0);

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
