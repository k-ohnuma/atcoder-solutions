import { useMutation } from "@tanstack/react-query";
import { solutionApi } from "@/features/solutions/api/solutionApi";
import { getFirebaseIdToken } from "@/lib/client/firebaseToken";
import { Solution } from "@/shared/model/solutionCreate";

export function useCreateSolution() {
  return useMutation({
    mutationFn: async (input: Solution) => {
      const token = await getFirebaseIdToken();
      const res = await solutionApi.create(input, token ?? "");
      return res;
    },
  });
}
