import { httpClient, Resp } from "@/lib/client/httpClient";
import { User, UserMe } from "@/server/domain/user";

export const userApi = {
  create: (userName: string, token: string): Promise<Resp<User>> =>
    httpClient.postWithToken<User>("api/users", token, JSON.stringify({ userName })),
  getMe: (token: string): Promise<Resp<UserMe>> => httpClient.getWithToken<UserMe>("api/users/me", token),
  revokeMe: (token: string): Promise<Resp<{ id: string }>> => httpClient.postWithToken<{ id: string }>("api/users/me", token),
  deleteMe: (token: string): Promise<Resp<{ id: string }>> => httpClient.deleteWithToken<{ id: string }>("api/users/me", token),
};
