"use client";

import { type Control, Controller, FieldPath, FieldValues } from "react-hook-form";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { fieldRootStyle } from "./style";

type Props<TFieldValues extends FieldValues> = {
  control: Control<TFieldValues>;
  name: FieldPath<TFieldValues>;
  label: string;
  type?: React.ComponentProps<"input">["type"];
  placeholder?: string;
  description?: string;
};

export function RhfTextField<TFieldValues extends FieldValues>({
  control,
  name,
  label,
  type = "text",
  placeholder,
  description,
}: Props<TFieldValues>) {
  return (
    <Controller
      control={control}
      name={name}
      render={({ field, fieldState }) => {
        return (
          <Field.Root className={fieldRootStyle} invalid={!!fieldState.error}>
            <Field.Label htmlFor={name}>{label}</Field.Label>
            <Input
              id={name}
              type={type}
              placeholder={placeholder}
              {...field}
              aria-invalid={fieldState.error ? "true" : undefined}
            />
            {description && <Field.HelperText>{description}</Field.HelperText>}
            {fieldState.error?.message && <Field.ErrorText>{fieldState.error.message}</Field.ErrorText>}
          </Field.Root>
        );
      }}
    />
  );
}
