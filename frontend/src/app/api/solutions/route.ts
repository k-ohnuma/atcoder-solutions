import { NextRequest, NextResponse } from "next/server";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";
import { getSolutionBySolutionIdQueryParams } from "@/server/interface/solution/get";
import { createSolutionBodySchema } from "@/server/interface/solution/post";

export async function POST(req: NextRequest) {
  const body = await req.json();
  const parsed = createSolutionBodySchema.safeParse(body);
  const header = req.headers;
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }
  const solution = parsed.data;
  const token = header.get("Authorization")?.replace(/^Bearer\s/, "");
  if (!token) {
    return NextResponse.json({ ok: false, error: "auth missing" }, { status: 401 });
  }
  const repo = new SolutionRepositoryImpl();
  const id = await repo.create(solution, token);
  if (!id.ok) {
    return NextResponse.json({ ok: false, error: id.error }, { status: id.status });
  }
  return NextResponse.json({ ok: true, data: id.data }, { status: 200 });
}

export async function GET(req: NextRequest) {
  const rawParams = Object.fromEntries(req.nextUrl.searchParams.entries());
  const parsed = getSolutionBySolutionIdQueryParams.safeParse(rawParams);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const repo = new SolutionRepositoryImpl();
  const solution = await repo.getBySolutionId(parsed.data.solutionId);
  if (!solution.ok) {
    return NextResponse.json({ ok: false, error: solution.error }, { status: solution.status });
  }
  return NextResponse.json({ ok: true, data: solution.data }, { status: 200 });
}
