import { useQuery } from "@tanstack/react-query";
import { ApiClient } from "../apiClient";
import { Problem } from "@/server/domain/problems";

const api = new ApiClient();

export const useProblemsByContest = (contest: string | null) => {
  return useQuery<Problem[]>({
    queryKey: ["problems", "byContest", contest],
    enabled: !!contest,
    queryFn: async () => await api.getProblemsByContest(contest!),
  });
}
