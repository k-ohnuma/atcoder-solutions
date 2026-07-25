import { NextRequest, NextResponse } from "next/server";
import { UserRepositoryImpl } from "@/server/infrastructure/repository/userRepository";
import { requireBearerToken } from "@/server/utils/bearerToken";

export async function GET(req: NextRequest) {
  const bearerToken = await requireBearerToken(req);
  if (!bearerToken.ok) {
    return NextResponse.json({ ok: false, error: bearerToken.error }, { status: bearerToken.status });
  }

  const repo = new UserRepositoryImpl();
  const me = await repo.getMe(bearerToken.data);
  if (!me.ok) {
    return NextResponse.json({ ok: false, error: me.error }, { status: me.status });
  }
  return NextResponse.json({ ok: true, data: me.data }, { status: 200 });
}

export async function DELETE(req: NextRequest) {
  const bearerToken = await requireBearerToken(req);
  if (!bearerToken.ok) {
    return NextResponse.json({ ok: false, error: bearerToken.error }, { status: bearerToken.status });
  }

  const repo = new UserRepositoryImpl();
  const deleted = await repo.deleteMe(bearerToken.data);
  if (!deleted.ok) {
    return NextResponse.json({ ok: false, error: deleted.error }, { status: deleted.status });
  }
  return NextResponse.json({ ok: true, data: deleted.data }, { status: 200 });
}

export async function POST(req: NextRequest) {
  const bearerToken = await requireBearerToken(req);
  if (!bearerToken.ok) {
    return NextResponse.json({ ok: false, error: bearerToken.error }, { status: bearerToken.status });
  }

  const repo = new UserRepositoryImpl();
  const revoked = await repo.revokeMe(bearerToken.data);
  if (!revoked.ok) {
    return NextResponse.json({ ok: false, error: revoked.error }, { status: revoked.status });
  }
  return NextResponse.json({ ok: true, data: revoked.data }, { status: 200 });
}
