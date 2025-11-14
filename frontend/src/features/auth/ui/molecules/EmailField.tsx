"use client";
import { type Control, FieldPath, type FieldValues } from "react-hook-form";
import { RhfTextField } from "../atoms/RhfTextField";

export function EmailField<T extends FieldValues>({ control }: { control: Control<T> }) {
  return (
    <RhfTextField<T>
      control={control}
      name={"email" as FieldPath<T>}
      label="メールアドレス"
      type="email"
      placeholder="email@example.com"
    />
  );
}
