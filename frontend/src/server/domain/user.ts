import { Resp } from "../response";

type Role = "user" | "admin";

export interface User {
  id: string;
  userName: string;
  role: Role;
}

export interface UserRepository {
  // findByUid(uid: string): Promise<User>;
  create(input: Pick<User, "userName">, token: string): Promise<Resp<User>>;
}
