import { NextRequest, NextResponse } from "next/server";
import { ProblemRepositoryImpl } from "@/server/infrastructure/repository/problemRepository";
import { getProblemsByContestSeriesQueryParams } from "@/server/interface/problem/get";

export async function GET(req: NextRequest) {
  const rawParams = Object.fromEntries(req.nextUrl.searchParams.entries());
  const parsed = getProblemsByContestSeriesQueryParams.safeParse(rawParams);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }
  const series = parsed.data.series;

  const repo = new ProblemRepositoryImpl();
  const problems = await repo.getProblemsByContestSeries(series);
  if (!problems.ok) {
    return NextResponse.json({ ok: false, error: problems.error }, { status: problems.status });
  }
  return NextResponse.json({ ok: true, data: problems.data }, { status: 200 });
}
