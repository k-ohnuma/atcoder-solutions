import { NextRequest, NextResponse } from "next/server";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";
import { getLatestSolutionsQueryParams } from "@/server/interface/solution/get";
import { createSolutionBodySchema } from "@/server/interface/solution/post";
import { authenticateRequest } from "@/server/utils/authRequest";

export async function POST(req: NextRequest) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const body = await req.json();
  const parsed = createSolutionBodySchema.safeParse(body);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const solution = parsed.data;
  const repo = new SolutionRepositoryImpl();
  const id = await repo.create(solution, authUser.data);
  if (!id.ok) {
    return NextResponse.json({ ok: false, error: id.error }, { status: id.status });
  }
  return NextResponse.json({ ok: true, data: id.data }, { status: 200 });
}

export async function GET(req: NextRequest) {
  const rawParams = Object.fromEntries(req.nextUrl.searchParams.entries());
  const parsed = getLatestSolutionsQueryParams.safeParse(rawParams);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const repo = new SolutionRepositoryImpl();
  const solutions = await repo.getLatest(parsed.data.limit);
  if (!solutions.ok) {
    return NextResponse.json({ ok: false, error: solutions.error }, { status: solutions.status });
  }
  return NextResponse.json({ ok: true, data: solutions.data }, { status: 200 });
}
