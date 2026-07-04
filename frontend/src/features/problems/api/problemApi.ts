import { httpClient, Resp } from "@/lib/client/httpClient";
import { ContestSeries, Problem } from "@/server/domain/problems";

export const problemApi = {
  getByContest: async (contest: string): Promise<Problem[]> => {
    const resp = await httpClient.get<Problem[]>("api/problems", { contest });
    if (resp.ok) {
      return resp.data;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return [];
  },

  getContestProblemGroupPage: ({
    series,
    limit,
    offset,
  }: {
    series: ContestSeries;
    limit?: number;
    offset?: number;
  }): Promise<
    Resp<{
      items: Record<string, Problem[]>;
      hasMore: boolean;
      totalContestCount: number;
    }>
  > => {
    const params = {
      series,
      ...(limit !== undefined ? { limit: String(limit) } : {}),
      ...(offset !== undefined ? { offset: String(offset) } : {}),
    };

    return httpClient.get<{
      items: Record<string, Problem[]>;
      hasMore: boolean;
      totalContestCount: number;
    }>("api/problems/contest-group", params);
  },
};
