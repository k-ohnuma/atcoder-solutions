import "server-only";
import { Contest, ContestRepository } from "@/server/domain/contests";
import { ContestSeries } from "@/server/domain/problems";
import { Resp } from "@/server/response";
import { BackendApiClient } from "@/server/utils/apiClient";
import { serverConfig } from "@/shared/config/backend";

export class ContestRepositoryImpl implements ContestRepository {
  private client: BackendApiClient;
  private static readonly READ_CACHE_SECONDS = 60 * 60;
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
    return await this.client.get<Contest[]>(path, params, ContestRepositoryImpl.READ_CACHE_SECONDS);
  };
}
