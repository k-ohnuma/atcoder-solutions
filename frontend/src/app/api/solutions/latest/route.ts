import { NextRequest, NextResponse } from "next/server";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";
import { getLatestSolutionsQueryParams } from "@/server/interface/solution/get";

export async function GET(req: NextRequest) {
  const rawParams = Object.fromEntries(req.nextUrl.searchParams.entries());
  const parsed = getLatestSolutionsQueryParams.safeParse(rawParams);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const repo = new SolutionRepositoryImpl();
  const solutions = await repo.getLatest(parsed.data.size);
  if (!solutions.ok) {
    return NextResponse.json({ ok: false, error: solutions.error }, { status: solutions.status });
  }

  return NextResponse.json({ ok: true, data: solutions.data }, { status: 200 });
}
