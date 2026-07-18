import { NextRequest, NextResponse } from "next/server";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";
import { updateSolutionBodySchema } from "@/server/interface/solution/post";
import { authenticateRequest } from "@/server/utils/authRequest";

type RouteContext = {
  params: Promise<{
    solutionId: string;
  }>;
};

export async function GET(_req: NextRequest, { params }: RouteContext) {
  const { solutionId } = await params;
  const repo = new SolutionRepositoryImpl();
  const solution = await repo.getBySolutionId(solutionId);
  if (!solution.ok) {
    return NextResponse.json({ ok: false, error: solution.error }, { status: solution.status });
  }
  return NextResponse.json({ ok: true, data: solution.data }, { status: 200 });
}

export async function PATCH(req: NextRequest, { params }: RouteContext) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const body = await req.json();
  const parsed = updateSolutionBodySchema.safeParse(body);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const { solutionId } = await params;
  const repo = new SolutionRepositoryImpl();
  const updated = await repo.update(solutionId, parsed.data, authUser.data);
  if (!updated.ok) {
    return NextResponse.json({ ok: false, error: updated.error }, { status: updated.status });
  }
  return NextResponse.json({ ok: true, data: updated.data }, { status: 200 });
}

export async function DELETE(req: NextRequest, { params }: RouteContext) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const { solutionId } = await params;
  const repo = new SolutionRepositoryImpl();
  const deleted = await repo.delete(solutionId, authUser.data);
  if (!deleted.ok) {
    return NextResponse.json({ ok: false, error: deleted.error }, { status: deleted.status });
  }
  return NextResponse.json({ ok: true, data: deleted.data }, { status: 200 });
}
