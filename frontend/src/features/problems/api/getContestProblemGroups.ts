import "server-only";

import { ContestProblemGroupPage, SupportedSeries } from "@/features/problems/model/contestProblemGroup";
import { ProblemRepositoryImpl } from "@/server/infrastructure/repository/problemRepository";

type GetContestProblemGroupsParams = {
  series: SupportedSeries;
  query?: string;
  limit?: number;
  offset?: number;
};

export async function getContestProblemGroups({
  series,
  query,
  limit,
  offset,
}: GetContestProblemGroupsParams): Promise<ContestProblemGroupPage> {
  const repo = new ProblemRepositoryImpl();
  const resp = await repo.getContestGroupByContestSeries({
    series,
    q: query,
    limit,
    offset,
  });
  if (!resp.ok) {
    throw new Error(`failed to fetch contest groups: status=${resp.status}, error=${resp.error}`);
  }

  return resp.data;
}
