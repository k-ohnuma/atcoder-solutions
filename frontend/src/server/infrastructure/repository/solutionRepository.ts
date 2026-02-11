import "server-only";

import { SolutionRepository } from "@/server/domain/solutions";
import { Resp } from "@/server/response";
import { BackendApiClient } from "@/server/utils/apiClient";
import { serverConfig } from "@/shared/config/backend";
import { SolutionDetail } from "@/shared/model/solution";
import { Solution, SolutionResponse } from "@/shared/model/solutionCreate";

export class SolutionRepositoryImpl implements SolutionRepository {
  private client: BackendApiClient;
  constructor() {
    const baseEndpoint = serverConfig.appConfig.apiBaseEndpoint;
    this.client = new BackendApiClient(baseEndpoint);
  }

  private createSolutionPath = () => {
    return "solutions";
  };

  create = async (solution: Solution, token: string): Promise<Resp<SolutionResponse>> => {
    const path = this.createSolutionPath();
    return await this.client.postWithToken<SolutionResponse>(path, token, JSON.stringify(solution));
  };

  getBySolutionId = async (solutionId: string): Promise<Resp<SolutionDetail>> => {
    const path = this.createSolutionPath();
    const params = { solutionId };
    return await this.client.get<SolutionDetail>(path, params);
  };
}
