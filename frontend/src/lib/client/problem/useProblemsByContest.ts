import { useQuery } from "@tanstack/react-query";
import { Problem } from "@/server/domain/problems";
import { ApiClient } from "../apiClient";

const api = new ApiClient();

export const useProblemsByContest = (contest: string | null) => {
  return useQuery<Problem[]>({
    queryKey: ["problems", "byContest", contest],
    enabled: !!contest,
    queryFn: async () => await api.getProblemsByContest(contest!),
  });
};
