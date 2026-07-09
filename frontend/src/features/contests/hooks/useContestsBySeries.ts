import { useQuery } from "@tanstack/react-query";
import { contestApi } from "@/features/contests/api/contestApi";
import { Contest } from "@/shared/model/contest";

export function useContestsBySeries(series: string | null) {
  return useQuery<Contest[]>({
    queryKey: ["contests", "bySeries", series],
    enabled: !!series,
    queryFn: async () => await contestApi.getBySeries(series!),
  });
}
