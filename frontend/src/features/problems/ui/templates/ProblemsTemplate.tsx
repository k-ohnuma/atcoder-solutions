import { ContestProblemGroup, SupportedSeries } from "@/features/problems/model/contestProblemGroup";
import { ShowMoreButton } from "../molecules/ShowMoreButton";
import { ContestProblemSections } from "../organisms/ContestProblemSections";
import { ProblemSearchForm } from "../organisms/ProblemSearchForm";
import { SeriesTabs } from "../organisms/SeriesTabs";

type ProblemsTemplateProps = {
  selectedSeries: SupportedSeries;
  query: string;
  totalMatchedProblems: number;
  errorMessage: string | null;
  groups: ContestProblemGroup[];
  canLoadMore: boolean;
  isLoadingMore: boolean;
  loadMoreError: string | null;
  onLoadMore: () => void;
};

export function ProblemsTemplate({
  selectedSeries,
  query,
  totalMatchedProblems,
  errorMessage,
  groups,
  canLoadMore,
  isLoadingMore,
  loadMoreError,
  onLoadMore,
}: ProblemsTemplateProps) {
  return (
    <>
      <div className="mb-6 flex flex-col gap-3">
        <SeriesTabs selectedSeries={selectedSeries} />
        <ProblemSearchForm selectedSeries={selectedSeries} query={query} />
        {errorMessage && <p className="text-sm text-destructive">{errorMessage}</p>}
        {query && !errorMessage && (
          <p className="text-sm text-muted-foreground">
            「{query}」の検索: {totalMatchedProblems} 件
          </p>
        )}
      </div>

      <ContestProblemSections groups={groups} />

      {!query && canLoadMore && (
        <div className="mt-6 flex flex-col items-center gap-2">
          <ShowMoreButton isLoading={isLoadingMore} onClickAction={onLoadMore} />
          {loadMoreError && <p className="text-sm text-destructive">{loadMoreError}</p>}
        </div>
      )}
    </>
  );
}
