import { z } from "zod";

export const commentFormSchema = z.object({
  bodyMd: z.string().trim().min(1, "コメントを入力してください").max(2000, "コメントは2000文字以内で入力してください"),
});

export type CommentFormInput = z.infer<typeof commentFormSchema>;
