import z from "zod";

export const createUserRequest = z.object({
  userName: z.string().min(1),
});

export type CreateUserRequest = z.infer<typeof createUserRequest>;
