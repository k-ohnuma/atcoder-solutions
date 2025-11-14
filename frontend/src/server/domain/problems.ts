import z from "zod";
import { Resp } from "../response";

export const contestSeries = z.enum(["ABC", "ARC", "AHC", "AGC", "OTHER"]);
export type ContestSeries = z.infer<typeof contestSeries>;

export interface Problem {
  id: string;
  contestCode: string;
  problemIndex: string;
  title: string;
}
export type ContestGroupCollection = Map<string, Problem[]>;

export interface ProblemRepository {
  getProblemsByContestSeries(series: ContestSeries): Promise<Resp<Problem[]>>;
  getContestGroupByContestSeries(series: ContestSeries): Promise<Resp<ContestGroupCollection>>;
}
