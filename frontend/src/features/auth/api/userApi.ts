import { httpClient, Resp } from "@/lib/client/httpClient";
import { User, UserMe } from "@/shared/model/user";

export const userApi = {
  create: (input: { userName: string; email: string; password: string }, turnstileToken: string | null): Promise<Resp<User>> =>
    httpClient.post<User>(
      "api/users",
      JSON.stringify(input),
      turnstileToken ? { "X-Turnstile-Token": turnstileToken } : undefined,
    ),
  getMe: (token: string): Promise<Resp<UserMe>> => httpClient.getWithToken<UserMe>("api/users/me", token),
  revokeMe: (token: string): Promise<Resp<{ id: string }>> => httpClient.postWithToken<{ id: string }>("api/users/me", token),
  deleteMe: (token: string): Promise<Resp<{ id: string }>> => httpClient.deleteWithToken<{ id: string }>("api/users/me", token),
};
