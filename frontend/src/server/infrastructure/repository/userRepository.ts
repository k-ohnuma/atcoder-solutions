import "server-only";
import { User, UserRepository } from "@/server/domain/user";
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

  async create(input: Pick<User, "userName">, token: string): Promise<Resp<User>> {
    const path = this.createReqPath();
    const body = {
      userName: input.userName,
    };
    const user = await this.client.postWithToken<User>(path, token, body);
    return user;
  }
}
