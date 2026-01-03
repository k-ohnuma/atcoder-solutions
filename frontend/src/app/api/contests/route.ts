import { NextRequest, NextResponse } from "next/server";
import { getContestsByContestSeriesQueryParams } from "@/server/interface/contest/get";
import { ContestRepositoryImpl } from "@/server/infrastructure/repository/contestRepository";

export async function GET(req: NextRequest) {
  const rawParams = Object.fromEntries(req.nextUrl.searchParams.entries());
  const parsed = getContestsByContestSeriesQueryParams.safeParse(rawParams);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }
  const series = parsed.data.series;

  const repo = new ContestRepositoryImpl();
  const problems = await repo.getContestsBySeries(series);
  if (!problems.ok) {
    return NextResponse.json({ ok: false, error: problems.error }, { status: problems.status });
  }
  return NextResponse.json({ ok: true, data: problems.data }, { status: 200 });
}
