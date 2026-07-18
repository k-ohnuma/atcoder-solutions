import "server-only";
import { ContestGroupPage, ContestSeries, Problem, ProblemRepository } from "@/server/domain/problems";
import { Resp } from "@/server/response";
import { BackendApiClient } from "@/server/utils/apiClient";
import { serverConfig } from "@/shared/config/backend";

export class ProblemRepositoryImpl implements ProblemRepository {
  private client: BackendApiClient;
  private static readonly READ_CACHE_SECONDS = 60 * 60;
  constructor() {
    const baseEndpoint = serverConfig.appConfig.apiBaseEndpoint;
    this.client = new BackendApiClient(baseEndpoint);
  }

  private getProblemsByContestPath = (contestCode: string) => {
    return `contests/${encodeURIComponent(contestCode)}/problems`;
  };
  private getProblemByIdPath = (problemId: string) => {
    return `problems/${encodeURIComponent(problemId)}`;
  };
  private getContestGroupByContestSeriesPath = () => {
    return "series";
  };

  getProblemById = async (problemId: string): Promise<Resp<Problem>> => {
    const path = this.getProblemByIdPath(problemId);
    return await this.client.get<Problem>(path, undefined, {
      kind: "no-store",
    });
  };

  getProblemsByContest = async (contestCode: string): Promise<Resp<Problem[]>> => {
    const path = this.getProblemsByContestPath(contestCode);
    return await this.client.get<Problem[]>(path, undefined, {
      kind: "revalidate",
      seconds: ProblemRepositoryImpl.READ_CACHE_SECONDS,
    });
  };
  getContestGroupByContestSeries = async ({
    series,
    q,
    limit,
    offset,
  }: {
    series: ContestSeries;
    q?: string;
    limit?: number;
    offset?: number;
  }): Promise<Resp<ContestGroupPage>> => {
    const params = {
      ...(q ? { q } : {}),
      ...(limit !== undefined ? { limit: String(limit) } : {}),
      ...(offset !== undefined ? { offset: String(offset) } : {}),
    };
    const path = `${this.getContestGroupByContestSeriesPath()}/${encodeURIComponent(series)}/problem-groups`;
    return await this.client.get<ContestGroupPage>(path, params, {
      kind: "revalidate",
      seconds: ProblemRepositoryImpl.READ_CACHE_SECONDS,
    });
  };
}
