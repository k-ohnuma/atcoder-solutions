import { NextRequest, NextResponse } from "next/server";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";
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
