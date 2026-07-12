import { useQuery } from "@tanstack/react-query";
import { problemApi } from "@/features/problems/api/problemApi";
import { Problem } from "@/shared/model/problem";

export const useProblemsByContest = (contest: string | null) => {
  return useQuery<Problem[]>({
    queryKey: ["problems", "byContest", contest],
    enabled: !!contest,
    queryFn: async () => await problemApi.getByContest(contest!),
  });
};
