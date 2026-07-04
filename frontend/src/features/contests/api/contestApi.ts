import { httpClient } from "@/lib/client/httpClient";
import { Contest } from "@/server/domain/contests";

export const contestApi = {
  getBySeries: async (series: string): Promise<Contest[]> => {
    const resp = await httpClient.get<Contest[]>("api/contests", { series });
    if (resp.ok) {
      return resp.data;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return [];
  },
};
