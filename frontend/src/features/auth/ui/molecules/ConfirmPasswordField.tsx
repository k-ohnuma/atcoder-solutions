"use client";
import { FieldPath, type Control, type FieldValues } from "react-hook-form";
import { RhfTextField } from "../atoms/RhfTextField";

export function ConfirmPasswordField<T extends FieldValues>({ control }: { control: Control<T> }) {
  return <RhfTextField<T> control={control} name={"confirm" as FieldPath<T>} label="パスワード（確認）" type="password" />;
}
