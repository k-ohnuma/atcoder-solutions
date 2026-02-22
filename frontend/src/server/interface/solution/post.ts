import z from "zod";

export const createSolutionBodySchema = z.object({
  problemId: z.string().trim().min(1),
  title: z.string().trim().min(1).max(120),
  bodyMd: z.string().trim().min(1).max(20000),
  submitUrl: z
    .string()
    .trim()
    .max(500)
    .refine((v) => v.length === 0 || /^https?:\/\//.test(v)),
  tags: z.array(z.string().trim().min(1).max(24)).max(6),
});

export const updateSolutionBodySchema = z.object({
  solutionId: z.string().uuid(),
  title: z.string().trim().min(1).max(120),
  bodyMd: z.string().trim().min(1).max(20000),
  submitUrl: z
    .string()
    .trim()
    .max(500)
    .refine((v) => v.length === 0 || /^https?:\/\//.test(v)),
  tags: z.array(z.string().trim().min(1).max(24)).max(6),
});

export const deleteSolutionQueryParams = z.object({
  solutionId: z.string().uuid(),
});
