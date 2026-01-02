import z from "zod";

export const signInSchema = z.object({
  email: z.email({ error: "メールアドレスの形式が正しくありません" }),
  password: z.string().min(1, "パスワードを入力してください"),
});
export type SignInSchema = z.infer<typeof signInSchema>;

export const signUpSchema = z
  .object({
    userName: z.string().min(1, "ユーザー名を入力してください"),
    email: z.email({ error: "メールアドレスの形式が正しくありません" }),
    password: z
      .string()
      .min(1, "パスワードを入力してください")
      .refine((val) => /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)[A-Za-z\d]{6,}$/.test(val), {
        message: "パスワードは6文字以上で、大文字・小文字・数字をすべて含めてください",
      }),
    confirm: z.string(),
  })
  .refine((v) => v.password === v.confirm, {
    path: ["confirm"],
    message: "パスワードが一致しません",
  });

export type SignUpSchema = z.infer<typeof signUpSchema>;
