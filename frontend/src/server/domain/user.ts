import { Resp } from "../response";

type Role = "user" | "admin";

export interface User {
  id: string;
  userName: string;
  role: Role;
}

export interface UserMe {
  id: string;
  userName: string;
}

export interface UserRepository {
  // findByUid(uid: string): Promise<User>;
  create(input: Pick<User, "userName">, token: string): Promise<Resp<User>>;
  getMe(token: string): Promise<Resp<UserMe>>;
  deleteMe(token: string): Promise<Resp<{ id: string }>>;
  revokeMe(token: string): Promise<Resp<{ id: string }>>;
}
