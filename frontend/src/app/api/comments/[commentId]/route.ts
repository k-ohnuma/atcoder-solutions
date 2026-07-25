import { NextRequest, NextResponse } from "next/server";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";
import { updateCommentBodySchema } from "@/server/interface/solution/comment";
import { requireBearerToken } from "@/server/utils/bearerToken";

type RouteContext = {
  params: Promise<{
    commentId: string;
  }>;
};

export async function PATCH(req: NextRequest, { params }: RouteContext) {
  const bearerToken = await requireBearerToken(req);
  if (!bearerToken.ok) {
    return NextResponse.json({ ok: false, error: bearerToken.error }, { status: bearerToken.status });
  }

  const body = await req.json();
  const parsed = updateCommentBodySchema.safeParse(body);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const { commentId } = await params;
  const repo = new SolutionRepositoryImpl();
  const updated = await repo.updateComment(commentId, parsed.data.bodyMd, bearerToken.data);
  if (!updated.ok) {
    return NextResponse.json({ ok: false, error: updated.error }, { status: updated.status });
  }

  return NextResponse.json({ ok: true, data: updated.data }, { status: 200 });
}

export async function DELETE(req: NextRequest, { params }: RouteContext) {
  const bearerToken = await requireBearerToken(req);
  if (!bearerToken.ok) {
    return NextResponse.json({ ok: false, error: bearerToken.error }, { status: bearerToken.status });
  }

  const { commentId } = await params;
  const repo = new SolutionRepositoryImpl();
  const deleted = await repo.deleteComment(commentId, bearerToken.data);
  if (!deleted.ok) {
    return NextResponse.json({ ok: false, error: deleted.error }, { status: deleted.status });
  }

  return NextResponse.json({ ok: true, data: deleted.data }, { status: 200 });
}
