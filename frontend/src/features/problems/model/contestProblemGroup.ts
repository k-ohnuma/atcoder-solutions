import { Problem } from "@/shared/model/problem";

export const supportedSeries = ["ABC", "ARC", "AGC", "AHC", "AWC", "OTHER"] as const;
export type SupportedSeries = (typeof supportedSeries)[number];

export type ContestProblemGroup = {
  contestId: string;
  problems: Problem[];
};

export type ContestProblemGroupPage = {
  groups: ContestProblemGroup[];
  hasMore: boolean;
  totalContestCount: number;
};

export function toContestProblemGroups(entries: Iterable<readonly [string, Problem[]]>): ContestProblemGroup[] {
  return [...entries].map(([contestId, problems]) => ({
    contestId,
    problems,
  }));
}

export function toContestProblemGroupPage({
  items,
  hasMore,
  totalContestCount,
}: {
  items: Iterable<readonly [string, Problem[]]>;
  hasMore: boolean;
  totalContestCount: number;
}): ContestProblemGroupPage {
  return {
    groups: toContestProblemGroups(items),
    hasMore,
    totalContestCount,
  };
}

export function normalizeSeries(value?: string): SupportedSeries {
  if (value && supportedSeries.includes(value as SupportedSeries)) {
    return value as SupportedSeries;
  }
  return "ABC";
}
