import { Resp } from "../response";
import { ContestSeries } from "./problems";

export interface Contest {
  code: string;
  seriesCode: string;
}

export interface ContestRepository {
  getContestsBySeries(contest: ContestSeries): Promise<Resp<Contest[]>>;
}
