import { useMutation } from "@tanstack/react-query";
import { ApiClient } from "@/lib/client/apiClient";
import { Solution } from "@/shared/model/solutionCreate";
import { getFirebaseIdToken } from "../firebaseToken";

const api = new ApiClient();

export function useCreateSolution() {
  return useMutation({
    mutationFn: async (input: Solution) => {
      const token = await getFirebaseIdToken();
      const res = await api.createSolution(input, token ?? "");
      return res;
    },
  });
}
