import z from "zod";

const userNamePattern = /^[A-Za-z0-9_]+$/;

export const createUserRequest = z.object({
  userName: z
    .string()
    .trim()
    .min(1, "userName is required")
    .max(120, "userName must be at most 120 characters")
    .regex(userNamePattern, "userName must contain only letters, numbers, and underscore"),
});

export type CreateUserRequest = z.infer<typeof createUserRequest>;
