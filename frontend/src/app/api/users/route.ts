import { NextRequest, NextResponse } from "next/server";
import { UserRepositoryImpl } from "@/server/infrastructure/repository/userRepository";
import { createUserRequest } from "@/server/interface/user/create";
import { authenticateRequest } from "@/server/utils/authRequest";

export async function POST(req: NextRequest) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }
  const json = await req.json();
  const parsed = createUserRequest.safeParse(json);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }
  const { userName } = parsed.data;

  const token = authUser.data;
  const repo = new UserRepositoryImpl();
  const created = await repo.create({ userName }, token);
  if (!created.ok) {
    return NextResponse.json({ ok: false, error: created.error }, { status: created.status });
  }
  return NextResponse.json({ ok: true, data: created.data }, { status: 200 });
}
