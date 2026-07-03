import { ContestProblemGroup, SupportedSeries } from "@/features/problems/model/contestProblemGroup";
import { ShowAllButton } from "../molecules/ShowAllButton";
import { ContestProblemSections } from "../organisms/ContestProblemSections";
import { ProblemSearchForm } from "../organisms/ProblemSearchForm";
import { SeriesTabs } from "../organisms/SeriesTabs";

type ProblemsTemplateProps = {
  selectedSeries: SupportedSeries;
  query: string;
  totalMatchedProblems: number;
  groups: ContestProblemGroup[];
  canLoadMore: boolean;
  isLoadingMore: boolean;
  onLoadMore: () => void;
};

export function ProblemsTemplate({
  selectedSeries,
  query,
  totalMatchedProblems,
  groups,
  canLoadMore,
  isLoadingMore,
  onLoadMore,
}: ProblemsTemplateProps) {
  return (
    <>
      <div className="mb-6 flex flex-col gap-3">
        <SeriesTabs selectedSeries={selectedSeries} />
        <ProblemSearchForm selectedSeries={selectedSeries} query={query} />
        {query && (
          <p className="text-sm text-muted-foreground">
            「{query}」の検索: {totalMatchedProblems} 件
          </p>
        )}
      </div>

      <ContestProblemSections groups={groups} />

      {!query && canLoadMore && (
        <div className="mt-6 flex justify-center">
          <ShowAllButton isLoading={isLoadingMore} onClick={onLoadMore} />
        </div>
      )}
    </>
  );
}
