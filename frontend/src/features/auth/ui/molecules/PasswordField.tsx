"use client";
import { type Control, FieldPath, type FieldValues } from "react-hook-form";
import { RhfTextField } from "../atoms/RhfTextField";

export function PasswordField<T extends FieldValues>({ control }: { control: Control<T> }) {
  return <RhfTextField<T> control={control} name={"password" as FieldPath<T>} label="パスワード" type="password" />;
}
