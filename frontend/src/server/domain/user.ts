import z from "zod";
import { Resp } from "../response";

export const colorScheme = z.enum([
  "red",
  "orange",
  "yellow",
  "blue",
  "cyan",
  "green",
  "brown",
  "gray",
]);
export type Color = z.infer<typeof colorScheme>;

type Role = "user" | "admin";

export interface User {
  id: string;
  userName: string;
  color: Color;
  role: Role;
}

export interface UserRepository {
  // findByUid(uid: string): Promise<User>;
  create(
    input: Pick<User, "userName" | "color">,
    token: string,
  ): Promise<Resp<User>>;
}
