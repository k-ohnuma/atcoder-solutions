import { httpClient } from "@/lib/client/httpClient";
import { Contest } from "@/shared/model/contest";

export const contestApi = {
  getBySeries: async (series: string): Promise<Contest[]> => {
    const resp = await httpClient.get<Contest[]>(`api/series/${encodeURIComponent(series)}/contests`);
    if (resp.ok) {
      return resp.data;
    }
    throw new Error(`failed to fetch contests by series: status=${resp.status}, error=${resp.error}`);
  },
};
