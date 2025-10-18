"use client";

import {
  Controller,
  type Control,
  FieldPath,
  FieldValues,
} from "react-hook-form";
import {
  Field,
  FieldLabel,
  FieldDescription,
  FieldError,
} from "@/components/ui/field";
import { Input } from "@/components/ui/input";

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
      render={({ field, fieldState }) => (
        <Field className="grid gap-1">
          <FieldLabel htmlFor={name}>{label}</FieldLabel>
          <Input
            id={name}
            type={type}
            placeholder={placeholder}
            {...field}
            aria-invalid={!!fieldState.error}
          />
          {description && <FieldDescription>{description}</FieldDescription>}
          {fieldState.error?.message && (
            <FieldError>{fieldState.error.message}</FieldError>
          )}
        </Field>
      )}
    />
  );
}
