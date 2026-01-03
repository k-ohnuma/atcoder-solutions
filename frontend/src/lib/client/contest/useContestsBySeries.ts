import { useQuery } from "@tanstack/react-query";
import { Contest } from "@/server/domain/contests";
import { ApiClient } from "../apiClient";

const api = new ApiClient();

export function useContestsBySeries(series: string | null) {
  return useQuery<Contest[]>({
    queryKey: ["contests", "bySeries", series],
    enabled: !!series,
    queryFn: async () => await api.getContestsByContestSeries(series!),
  });
}
