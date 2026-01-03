import "server-only";
import { Resp } from "@/server/response";
import { BackendApiClient } from "@/server/utils/apiClient";
import { serverConfig } from "@/shared/config/backend";
import { Contest, ContestRepository } from "@/server/domain/contests";
import { ContestSeries } from "@/server/domain/problems";

export class ContestRepositoryImpl implements ContestRepository {
  private client: BackendApiClient;
  constructor() {
    const baseEndpoint = serverConfig.appConfig.apiBaseEndpoint;
    this.client = new BackendApiClient(baseEndpoint);
  }

  private getContestsByContestSeriesPath = () => {
    return "contests";
  };

  getContestsBySeries = async (series: ContestSeries): Promise<Resp<Contest[]>> => {
    const params = { series };
    const path = this.getContestsByContestSeriesPath();
    return await this.client.get<Contest[]>(path, params);
  };
}

