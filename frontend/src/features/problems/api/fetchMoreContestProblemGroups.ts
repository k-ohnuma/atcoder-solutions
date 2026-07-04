import {
  ContestProblemGroupPage,
  SupportedSeries,
  toContestProblemGroupPage,
} from "@/features/problems/model/contestProblemGroup";
import { Problem } from "@/shared/model/problem";

type FetchContestProblemGroupsParams = {
  series: SupportedSeries;
  limit?: number;
  offset?: number;
};

type ContestProblemGroupsResponse = {
  ok?: boolean;
  data?: {
    items: Record<string, Problem[]>;
    hasMore: boolean;
    totalContestCount: number;
  };
  error?: string;
};

export async function fetchMoreContestProblemGroups({
  series,
  limit,
  offset,
}: FetchContestProblemGroupsParams): Promise<ContestProblemGroupPage> {
  const url = new URL("/api/problems/contest-group", window.location.origin);
  url.searchParams.set("series", series);
  if (limit !== undefined) {
    url.searchParams.set("limit", String(limit));
  }
  if (offset !== undefined) {
    url.searchParams.set("offset", String(offset));
  }

  const res = await fetch(url, {
    method: "GET",
    headers: {
      Accept: "application/json",
    },
  });
  const json = (await res.json()) as ContestProblemGroupsResponse;
  if (!res.ok || !json.ok || !json.data) {
    throw new Error(`failed to fetch contest groups: status=${res.status}, error=${json.error ?? "unknown error"}`);
  }

  return toContestProblemGroupPage({
    items: Object.entries(json.data.items),
    hasMore: json.data.hasMore,
    totalContestCount: json.data.totalContestCount,
  });
}
