import { NextRequest, NextResponse } from "next/server";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";
import { authenticateRequest } from "@/server/utils/authRequest";

type RouteContext = {
  params: Promise<{
    solutionId: string;
  }>;
};

export async function GET(req: NextRequest, { params }: RouteContext) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const { solutionId } = await params;
  const repo = new SolutionRepositoryImpl();
  const status = await repo.getMyVoteStatus(solutionId, authUser.data);
  if (!status.ok) {
    return NextResponse.json({ ok: false, error: status.error }, { status: status.status });
  }

  return NextResponse.json({ ok: true, data: status.data }, { status: 200 });
}

export async function PUT(req: NextRequest, { params }: RouteContext) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const { solutionId } = await params;
  const repo = new SolutionRepositoryImpl();
  const voted = await repo.vote(solutionId, authUser.data);
  if (!voted.ok) {
    return NextResponse.json({ ok: false, error: voted.error }, { status: voted.status });
  }

  return NextResponse.json({ ok: true, data: voted.data }, { status: 200 });
}

export async function DELETE(req: NextRequest, { params }: RouteContext) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const { solutionId } = await params;
  const repo = new SolutionRepositoryImpl();
  const unvoted = await repo.unvote(solutionId, authUser.data);
  if (!unvoted.ok) {
    return NextResponse.json({ ok: false, error: unvoted.error }, { status: unvoted.status });
  }

  return NextResponse.json({ ok: true, data: unvoted.data }, { status: 200 });
}
