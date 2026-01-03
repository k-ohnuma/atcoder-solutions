import z from "zod";
import { colorScheme } from "@/server/domain/user";

export const createUserRequest = z.object({
  userName: z.string().min(1),
  color: colorScheme,
});

export type CreateUserRequest = z.infer<typeof createUserRequest>;
