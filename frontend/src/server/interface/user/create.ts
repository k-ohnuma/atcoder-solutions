import z from "zod";

export const createUserRequest = z.object({
  userName: z.string().trim().min(1).max(120),
});

export type CreateUserRequest = z.infer<typeof createUserRequest>;
