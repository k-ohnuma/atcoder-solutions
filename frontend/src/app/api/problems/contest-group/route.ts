import { NextRequest, NextResponse } from "next/server";
import { publicCacheHeaders } from "@/server/http/cache";
import { ProblemRepositoryImpl } from "@/server/infrastructure/repository/problemRepository";
import { getProblemsByContestSeriesQueryParams } from "@/server/interface/problem/get";

export async function GET(req: NextRequest) {
  const rawParams = Object.fromEntries(req.nextUrl.searchParams.entries());
  const parsed = getProblemsByContestSeriesQueryParams.safeParse(rawParams);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }
  const { series, q, limit, offset } = parsed.data;

  const repo = new ProblemRepositoryImpl();
  const problems = await repo.getContestGroupByContestSeries({ series, q, limit, offset });
  if (!problems.ok) {
    return NextResponse.json({ ok: false, error: problems.error }, { status: problems.status });
  }
  const headers = q?.trim() ? { "Cache-Control": "no-store" } : publicCacheHeaders(3600, 300);
  return NextResponse.json(
    { ok: true, data: { ...problems.data, items: Object.fromEntries(problems.data.items) } },
    { status: 200, headers },
  );
}
