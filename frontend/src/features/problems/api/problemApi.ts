import { httpClient } from "@/lib/client/httpClient";
import { Problem } from "@/shared/model/problem";
import { ContestProblemGroupPage, SupportedSeries } from "../model/contestProblemGroup";

export const problemApi = {
  getByContest: async (contest: string): Promise<Problem[]> => {
    const resp = await httpClient.get<Problem[]>(`api/contests/${encodeURIComponent(contest)}/problems`);
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
      ...(limit !== undefined ? { limit: String(limit) } : {}),
      ...(offset !== undefined ? { offset: String(offset) } : {}),
    };

    const resp = await httpClient.get<{
      groups: { contestId: string; problems: Problem[] }[];
      hasMore: boolean;
      totalContestCount: number;
    }>(`api/series/${encodeURIComponent(series)}/problem-groups`, params);

    if (!resp.ok) {
      throw new Error(`failed to fetch contest groups: status=${resp.status}, error=${resp.error}`);
    }

    return resp.data;
  },
};
