import { colorScheme } from "@/server/domain/user";
import z from "zod";

export const createUserRequest = z.object({
  userName: z.string().min(1),
  color: colorScheme
});

export type CreateUserRequest = z.infer<typeof createUserRequest>;
