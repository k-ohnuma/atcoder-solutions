import { NextRequest } from "next/server";
import { Resp } from "../response";

export async function requireBearerToken(request: NextRequest): Promise<Resp<string>> {
  const authHeader = request.headers.get("Authorization");

  if (!authHeader || !authHeader.startsWith("Bearer ")) {
    return {
      status: 401,
      error: "認証できません: トークンが不正です",
      ok: false,
    };
  }
  const token = authHeader.split(" ")[1];

  return {
    data: token,
    status: 200,
    ok: true,
  };
}
