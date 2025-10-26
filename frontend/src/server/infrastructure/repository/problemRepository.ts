import "server-only";
import { BackendApiClient } from "@/server/utils/apiClient";
import { backendConfig } from "@/shared/config/backend";
import { Resp } from "@/server/response";
import {
  ContestSeries,
  Problem,
  ProblemRepository,
} from "@/server/domain/problems";

export class ProblemRepositoryImpl implements ProblemRepository {
  private client: BackendApiClient;
  constructor() {
    const baseEndpoint = backendConfig.appConfig.apiBaseEndpoint;
    this.client = new BackendApiClient(baseEndpoint);
  }

  private getProblemsByContestSeriesPath = () => {
    return "problems";
  };

  getProblemsByContestSeries = async (
    series: ContestSeries,
  ): Promise<Resp<Problem[]>> => {
    const params = { series };
    const path = this.getProblemsByContestSeriesPath();
    return await this.client.get<Problem[]>(path, params);
  };
}
