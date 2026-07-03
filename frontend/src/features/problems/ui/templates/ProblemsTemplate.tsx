"use client";

import Link from "next/link";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Problem } from "@/shared/model/problem";
import { ShowAllButton } from "../molecules/ShowAllButton";
import { ContestProblemSections } from "../organisms/ContestProblemSections";

const supportedSeries = ["ABC", "ARC", "AGC", "AHC", "AWC", "OTHER"] as const;
type SupportedSeries = (typeof supportedSeries)[number];

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
  visibleContests: Array<readonly [string, Problem[]]>;
  hasMore: boolean;
  pageSize: number;
}) {
  const [contests, setContests] = useState<Array<readonly [string, Problem[]]>>(visibleContests);
  const [canLoadMore, setCanLoadMore] = useState(hasMore);
  const [isLoadingMore, setIsLoadingMore] = useState(false);

  useEffect(() => {
    setContests(visibleContests);
    setCanLoadMore(hasMore);
  }, [visibleContests, hasMore]);

  const loadMore = async () => {
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
      const data = json.data;

      setContests((current) => [...current, ...Object.entries(data.items)]);
      setCanLoadMore(data.hasMore);
    } finally {
      setIsLoadingMore(false);
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
