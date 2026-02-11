"use client";

import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import type { Contest } from "@/server/domain/contests";
import type { Problem } from "@/shared/model/problem";
import { ContestCombobox } from "./ContestCombobox";
import { ProblemButtonSelect } from "./ProblemButtonSelect";

type ContestProblemSelectorSectionProps = {
  series: string;
  onSeriesChangeAction: (value: string) => void;
  contests: Contest[];
  contestsLoading: boolean;
  contestsIsError: boolean;
  contestId: string | null;
  onContestChangeAction: (value: string | null) => void;
  problems: Problem[];
  problemsLoading: boolean;
  problemsIsError: boolean;
  selectedProblemId: string | null;
  selectedProblem: Problem | null;
  onProblemChangeAction: (value: string) => void;
  problemErrorMessage?: string;
};

export function ContestProblemSelectorSection({
  series,
  onSeriesChangeAction,
  contests,
  contestsLoading,
  contestsIsError,
  contestId,
  onContestChangeAction,
  problems,
  problemsLoading,
  problemsIsError,
  selectedProblemId,
  selectedProblem,
  onProblemChangeAction,
  problemErrorMessage,
}: ContestProblemSelectorSectionProps) {
  return (
    <div className="rounded-2xl border border-border bg-card p-4 shadow-sm">
      <div className="grid gap-3 md:grid-cols-2">
        <div className="space-y-2">
          <div className="text-xs font-medium text-muted-foreground">コンテスト種別</div>
          <Tabs value={series} onValueChange={onSeriesChangeAction}>
            <TabsList className="grid w-full grid-cols-4">
              <TabsTrigger value="ABC">ABC</TabsTrigger>
              <TabsTrigger value="ARC">ARC</TabsTrigger>
              <TabsTrigger value="AGC">AGC</TabsTrigger>
              <TabsTrigger value="OTHER">OTHER</TabsTrigger>
            </TabsList>
          </Tabs>
        </div>

        <div className="space-y-2">
          <div className="text-xs font-medium text-muted-foreground">コンテスト名</div>
          <ContestCombobox
            items={contests}
            value={contestId}
            onChangeAction={onContestChangeAction}
            isLoading={contestsLoading}
            disabled={contestsLoading}
            placeholder="コンテストを選択"
          />
          {contestsIsError && <div className="text-sm text-destructive">コンテスト取得に失敗</div>}
        </div>
      </div>

      <div className="mt-4 space-y-2">
        <div className="flex items-center justify-between">
          <div className="text-xs font-medium text-muted-foreground">問題</div>
          {selectedProblem && (
            <div className="text-xs text-muted-foreground">
              選択中: <span className="font-medium text-foreground">{selectedProblem.id}</span>
            </div>
          )}
        </div>

        <ProblemButtonSelect
          items={problems}
          value={selectedProblemId}
          onChangeAction={onProblemChangeAction}
          disabled={!contestId || problemsLoading}
          isLoading={problemsLoading}
        />
        {problemErrorMessage && <p className="text-xs text-destructive">{problemErrorMessage}</p>}
        {problemsIsError && <div className="text-sm text-destructive">問題取得に失敗</div>}
      </div>
    </div>
  );
}
