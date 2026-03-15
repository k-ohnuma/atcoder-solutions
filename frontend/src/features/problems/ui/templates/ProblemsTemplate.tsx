import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { ShowAllButton } from "../molecules/ShowAllButton";
import { ContestProblemSections } from "../organisms/ContestProblemSections";

const supportedSeries = ["ABC", "ARC", "AGC", "AHC", "OTHER"] as const;
type SupportedSeries = (typeof supportedSeries)[number];

export function ProblemsTemplate({
  selectedSeries,
  query,
  totalMatchedProblems,
  visibleContests,
  shouldShowAll,
  remainingContestCount,
}: {
  selectedSeries: SupportedSeries;
  query: string;
  totalMatchedProblems: number;
  visibleContests: Array<readonly [string, import("@/shared/model/problem").Problem[]]>;
  shouldShowAll: boolean;
  remainingContestCount: number;
}) {
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
            「{query}」の検索: {totalMatchedProblems} 件
          </p>
        )}
      </section>

      <ContestProblemSections contests={visibleContests} />

      {!shouldShowAll && remainingContestCount > 0 && (
        <section className="mt-6 flex justify-center">
          <ShowAllButton href={`/?series=${selectedSeries}&showAll=1`} remainingContestCount={remainingContestCount} />
        </section>
      )}
    </>
  );
}
