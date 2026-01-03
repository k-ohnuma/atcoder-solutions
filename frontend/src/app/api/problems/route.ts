import { NextRequest, NextResponse } from "next/server";
import { ProblemRepositoryImpl } from "@/server/infrastructure/repository/problemRepository";
import { getProblemsByContestQueryParams } from "@/server/interface/problem/get";

export async function GET(req: NextRequest) {
  const rawParams = Object.fromEntries(req.nextUrl.searchParams.entries());
  const parsed = getProblemsByContestQueryParams.safeParse(rawParams);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }
  const contest = parsed.data.contest;

  const repo = new ProblemRepositoryImpl();
  const problems = await repo.getProblemsByContest(contest);
  if (!problems.ok) {
    return NextResponse.json({ ok: false, error: problems.error }, { status: problems.status });
  }
  return NextResponse.json({ ok: true, data: problems.data }, { status: 200 });
}
