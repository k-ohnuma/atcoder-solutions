export type Role = "user" | "admin";

export interface User {
  id: string;
  userName: string;
  role: Role;
}

export interface UserMe {
  id: string;
  userName: string;
}
