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

  private getProblemsByContestSeriesPath = () => {
    return "problems";
  };
  private getProblemByIdPath = (problemId: string) => {
    return `problems/${encodeURIComponent(problemId)}`;
  };
  private getContestGroupByContestSeriesPath = () => {
    return "problems/contest-group";
  };

  getProblemById = async (problemId: string): Promise<Resp<Problem>> => {
    const path = this.getProblemByIdPath(problemId);
    return await this.client.get<Problem>(path, undefined, {
      kind: "no-store",
    });
  };

  getProblemsByContest = async (contest: string): Promise<Resp<Problem[]>> => {
    const params = { contest };
    const path = this.getProblemsByContestSeriesPath();
    return await this.client.get<Problem[]>(path, params, {
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
      series,
      ...(q ? { q } : {}),
      ...(limit !== undefined ? { limit: String(limit) } : {}),
      ...(offset !== undefined ? { offset: String(offset) } : {}),
    };
    const path = this.getContestGroupByContestSeriesPath();
    const resp = await this.client.get<{
      items: Record<string, Problem[]>;
      hasMore: boolean;
      totalContestCount: number;
    }>(path, params, {
      kind: "revalidate",
      seconds: ProblemRepositoryImpl.READ_CACHE_SECONDS,
    });
    if (!resp.ok) {
      return resp;
    }

    return {
      ok: true,
      status: resp.status,
      data: {
        ...resp.data,
        items: new Map<string, Problem[]>(Object.entries(resp.data.items)),
      },
    };
  };
}
