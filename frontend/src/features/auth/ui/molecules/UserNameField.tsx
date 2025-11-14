"use client";
import { type Control, FieldPath, type FieldValues } from "react-hook-form";
import { RhfTextField } from "../atoms/RhfTextField";

export function UserNameField<T extends FieldValues>({ control }: { control: Control<T> }) {
  return (
    <RhfTextField<T>
      control={control}
      name={"userName" as FieldPath<T>}
      label="ユーザー名"
      placeholder="chokudai"
      description="他のユーザーに装う名称はご遠慮ください"
    />
  );
}
