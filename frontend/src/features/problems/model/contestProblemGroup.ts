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

export function normalizeSeries(value?: string): SupportedSeries {
  if (value && supportedSeries.includes(value as SupportedSeries)) {
    return value as SupportedSeries;
  }
  return "ABC";
}
