import { NextRequest, NextResponse } from "next/server";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";
import { createCommentBodySchema } from "@/server/interface/solution/comment";
import { requireBearerToken } from "@/server/utils/bearerToken";

type RouteContext = {
  params: Promise<{
    solutionId: string;
  }>;
};

export async function GET(_req: NextRequest, { params }: RouteContext) {
  const { solutionId } = await params;
  const repo = new SolutionRepositoryImpl();
  const comments = await repo.getCommentsBySolutionId(solutionId);
  if (!comments.ok) {
    return NextResponse.json({ ok: false, error: comments.error }, { status: comments.status });
  }

  return NextResponse.json({ ok: true, data: comments.data }, { status: 200 });
}

export async function POST(req: NextRequest, { params }: RouteContext) {
  const bearerToken = await requireBearerToken(req);
  if (!bearerToken.ok) {
    return NextResponse.json({ ok: false, error: bearerToken.error }, { status: bearerToken.status });
  }

  const body = await req.json();
  const parsed = createCommentBodySchema.safeParse(body);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const { solutionId } = await params;
  const repo = new SolutionRepositoryImpl();
  const created = await repo.createComment(solutionId, parsed.data.bodyMd, bearerToken.data);
  if (!created.ok) {
    return NextResponse.json({ ok: false, error: created.error }, { status: created.status });
  }

  return NextResponse.json({ ok: true, data: created.data }, { status: 200 });
}
