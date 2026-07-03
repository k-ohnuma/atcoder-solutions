"use client";

import Link from "next/link";
import { useEffect, useRef, useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Problem } from "@/shared/model/problem";
import { ShowAllButton } from "../molecules/ShowAllButton";
import { ContestProblemSections } from "../organisms/ContestProblemSections";

const supportedSeries = ["ABC", "ARC", "AGC", "AHC", "AWC", "OTHER"] as const;
type SupportedSeries = (typeof supportedSeries)[number];
type ContestEntry = readonly [string, Problem[]];

function appendUniqueContestEntries(current: ContestEntry[], incoming: ContestEntry[]): ContestEntry[] {
  const existingContestIds = new Set(current.map(([contestId]) => contestId));
  const next = [...current];
  for (const entry of incoming) {
    const [contestId] = entry;
    if (existingContestIds.has(contestId)) {
      continue;
    }
    existingContestIds.add(contestId);
    next.push(entry);
  }
  return next;
}

export function ProblemsTemplate({
  selectedSeries,
  query,
  totalMatchedProblems,
  visibleContests,
  hasMore,
  pageSize,
}: {
  selectedSeries: SupportedSeries;
  query: string;
  totalMatchedProblems: number;
  visibleContests: ContestEntry[];
  hasMore: boolean;
  pageSize: number;
}) {
  const [contests, setContests] = useState<ContestEntry[]>(visibleContests);
  const [canLoadMore, setCanLoadMore] = useState(hasMore);
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const requestVersionRef = useRef(0);
  const isLoadingMoreRef = useRef(false);

  useEffect(() => {
    requestVersionRef.current += 1;
    isLoadingMoreRef.current = false;
    setContests(visibleContests);
    setCanLoadMore(hasMore);
    setIsLoadingMore(false);
  }, [visibleContests, hasMore]);

  const loadMore = async () => {
    if (isLoadingMoreRef.current) {
      return;
    }
    isLoadingMoreRef.current = true;
    const requestVersion = requestVersionRef.current;
    setIsLoadingMore(true);
    try {
      const url = new URL("/api/problems/contest-group", window.location.origin);
      url.searchParams.set("series", selectedSeries);
      url.searchParams.set("limit", String(pageSize));
      url.searchParams.set("offset", String(contests.length));

      const res = await fetch(url, {
        headers: {
          Accept: "application/json",
        },
      });
      const json = (await res.json()) as {
        ok?: boolean;
        data?: {
          items: Record<string, Problem[]>;
          hasMore: boolean;
        };
      };
      if (!res.ok || !json.ok || !json.data) {
        return;
      }
      if (requestVersion !== requestVersionRef.current) {
        return;
      }
      const data = json.data;
      const incomingContests = Object.entries(data.items);

      setContests((current) => appendUniqueContestEntries(current, incomingContests));
      setCanLoadMore(data.hasMore);
    } finally {
      if (requestVersion === requestVersionRef.current) {
        isLoadingMoreRef.current = false;
        setIsLoadingMore(false);
      }
    }
  };

  const displayedProblemCount = contests.reduce((acc, [, problems]) => acc + problems.length, 0);

  return (
    <>
      <section className="mb-6 flex flex-col gap-3">
        <div className="flex flex-wrap gap-2">
          {supportedSeries.map((code) => {
            const active = code === selectedSeries;
            return (
              <Button key={code} asChild size="sm" variant={active ? "default" : "outline"}>
                <Link href={`/?series=${code}`}>{code}</Link>
              </Button>
            );
          })}
        </div>

        <form method="GET" className="flex w-full items-center gap-2">
          <input type="hidden" name="series" value={selectedSeries} />
          <Input name="q" placeholder="検索" defaultValue={query} />
          <Button type="submit" variant="outline">
            検索
          </Button>
          {query && (
            <Button asChild type="button" variant="ghost">
              <Link href={`/?series=${selectedSeries}`}>クリア</Link>
            </Button>
          )}
        </form>
        {query && (
          <p className="text-sm text-muted-foreground">
            「{query}」の検索: {displayedProblemCount || totalMatchedProblems} 件
          </p>
        )}
      </section>

      <ContestProblemSections contests={contests} />

      {!query && canLoadMore && (
        <section className="mt-6 flex justify-center">
          <ShowAllButton isLoading={isLoadingMore} onClick={loadMore} />
        </section>
      )}
    </>
  );
}
