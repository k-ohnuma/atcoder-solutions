import "server-only";
import { User, UserMe, UserRepository } from "@/server/domain/user";
import { Resp } from "@/server/response";
import { BackendApiClient } from "@/server/utils/apiClient";
import { serverConfig } from "@/shared/config/backend";

export class UserRepositoryImpl implements UserRepository {
  private client: BackendApiClient;
  constructor() {
    const baseEndpoint = serverConfig.appConfig.apiBaseEndpoint;
    this.client = new BackendApiClient(baseEndpoint);
  }

  private createReqPath = () => {
    return "users";
  };
  private mePath = () => {
    return "users/me";
  };
  private revokeMePath = () => {
    return "users/me/revoke";
  };

  async create(input: Pick<User, "userName">, token: string): Promise<Resp<User>> {
    const path = this.createReqPath();
    const body = {
      userName: input.userName,
    };
    const user = await this.client.postWithToken<User>(path, token, JSON.stringify(body));
    return user;
  }

  async getMe(token: string): Promise<Resp<UserMe>> {
    const path = this.mePath();
    return await this.client.getWithToken<UserMe>(path, token);
  }

  async deleteMe(token: string): Promise<Resp<{ id: string }>> {
    const path = this.mePath();
    return await this.client.deleteWithToken<{ id: string }>(path, token);
  }

  async revokeMe(token: string): Promise<Resp<{ id: string }>> {
    const path = this.revokeMePath();
    return await this.client.postWithToken<{ id: string }>(path, token, "{}");
  }
}
