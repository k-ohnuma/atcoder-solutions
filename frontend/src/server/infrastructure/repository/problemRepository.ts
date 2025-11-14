import "server-only";
import { ContestGroupCollection, ContestSeries, Problem, ProblemRepository } from "@/server/domain/problems";
import { Resp } from "@/server/response";
import { BackendApiClient } from "@/server/utils/apiClient";
import { serverConfig } from "@/shared/config/backend";

export class ProblemRepositoryImpl implements ProblemRepository {
  private client: BackendApiClient;
  constructor() {
    const baseEndpoint = serverConfig.appConfig.apiBaseEndpoint;
    this.client = new BackendApiClient(baseEndpoint);
  }

  private getProblemsByContestSeriesPath = () => {
    return "problems";
  };
  private getContestGroupByContestSeriesPath = () => {
    return "problems/contest-group";
  };

  getProblemsByContestSeries = async (series: ContestSeries): Promise<Resp<Problem[]>> => {
    const params = { series };
    const path = this.getProblemsByContestSeriesPath();
    return await this.client.get<Problem[]>(path, params);
  };
  getContestGroupByContestSeries = async (series: ContestSeries): Promise<Resp<ContestGroupCollection>> => {
    const params = { series };
    const path = this.getContestGroupByContestSeriesPath();
    return await this.client.get<ContestGroupCollection>(path, params);
  };
}
