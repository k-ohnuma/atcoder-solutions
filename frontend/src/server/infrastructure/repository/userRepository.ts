import "server-only";
import { User, UserRepository } from "@/server/domain/user";
import { BackendApiClient } from "@/server/utils/apiClient";
import { backendConfig } from "@/shared/config/backend";
import { Resp } from "@/server/response";

export class UserRepositoryImpl implements UserRepository {
  private client: BackendApiClient;
  constructor() {
    const baseEndpoint = backendConfig.appConfig.apiBaseEndpoint;
    this.client = new BackendApiClient(baseEndpoint);
  }

  private createReqPath = () => {
    return 'users';
  }

  async create(input: Pick<User, 'userName' | 'color'>, token: string): Promise<Resp<User>> {
    const path = this.createReqPath();
    const body = {
      userName: input.userName,
      color: input.color
    }
    const user = await this.client.postWithToken<User>(path, token, body)
    return user
  }

}
