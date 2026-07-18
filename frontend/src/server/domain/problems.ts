import z from "zod";
import { Resp } from "../response";

export const contestSeries = z.enum(["ABC", "ARC", "AGC", "AHC", "AWC", "OTHER"]);
export type ContestSeries = z.infer<typeof contestSeries>;

export interface Problem {
  id: string;
  contestCode: string;
  problemIndex: string;
  title: string;
  difficulty?: number | null;
}
export interface ContestProblemGroup {
  contestId: string;
  problems: Problem[];
}
export interface ContestGroupPage {
  groups: ContestProblemGroup[];
  hasMore: boolean;
  totalContestCount: number;
}

export interface ProblemRepository {
  getProblemById(problemId: string): Promise<Resp<Problem>>;
  getProblemsByContest(contest: string): Promise<Resp<Problem[]>>;
  getContestGroupByContestSeries(params: {
    series: ContestSeries;
    q?: string;
    limit?: number;
    offset?: number;
  }): Promise<Resp<ContestGroupPage>>;
}
