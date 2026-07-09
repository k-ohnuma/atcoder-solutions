import { httpClient } from "@/lib/client/httpClient";
import { Problem } from "@/shared/model/problem";
import { ContestProblemGroupPage, SupportedSeries, toContestProblemGroupPage } from "../model/contestProblemGroup";

export const problemApi = {
  getByContest: async (contest: string): Promise<Problem[]> => {
    const resp = await httpClient.get<Problem[]>("api/problems", { contest });
    if (resp.ok) {
      return resp.data;
    }
    throw new Error(`failed to fetch problems by contest: status=${resp.status}, error=${resp.error}`);
  },

  getContestProblemGroupPage: async ({
    series,
    limit,
    offset,
  }: {
    series: SupportedSeries;
    limit?: number;
    offset?: number;
  }): Promise<ContestProblemGroupPage> => {
    const params = {
      series,
      ...(limit !== undefined ? { limit: String(limit) } : {}),
      ...(offset !== undefined ? { offset: String(offset) } : {}),
    };

    const resp = await httpClient.get<{
      items: Record<string, Problem[]>;
      hasMore: boolean;
      totalContestCount: number;
    }>("api/problems/contest-group", params);

    if (!resp.ok) {
      throw new Error(`failed to fetch contest groups: status=${resp.status}, error=${resp.error}`);
    }

    return toContestProblemGroupPage({
      items: Object.entries(resp.data.items),
      hasMore: resp.data.hasMore,
      totalContestCount: resp.data.totalContestCount,
    });
  },
};
