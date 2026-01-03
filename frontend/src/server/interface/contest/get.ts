import z from "zod";
import { contestSeries } from "@/server/domain/problems";

export const getContestsByContestSeriesQueryParams = z.object({
  series: contestSeries,
});

export type GetContestsByContestSeriesQueryParams = z.infer<typeof getContestsByContestSeriesQueryParams>;
