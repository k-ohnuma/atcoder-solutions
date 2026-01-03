import { useQuery } from "@tanstack/react-query";
import { ApiClient } from "../apiClient";
import { Contest } from "@/server/domain/contests";

const api = new ApiClient();

export function useContestsBySeries(series: string | null) {
  return useQuery<Contest[]>({
    queryKey: ["contests", "bySeries", series],
    enabled: !!series,
    queryFn: async () => await api.getContestsByContestSeries(series!),
  });
}
