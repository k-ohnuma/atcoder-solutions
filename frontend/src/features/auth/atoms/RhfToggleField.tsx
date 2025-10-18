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
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { cn } from "@/lib/utils";

type ToggleItem = {
  label: string;
  value: string;
  dot?: string;
  on?: string;
};
type Props<TFieldValues extends FieldValues> = {
  control: Control<TFieldValues>;
  name: FieldPath<TFieldValues>;
  label: string;
  toggleItems: ToggleItem[];
  toggleType: "single";
  type?: React.ComponentProps<"input">["type"];
  placeholder?: string;
  description?: string;
};

export function RhfToggleField<TFieldValues extends FieldValues>({
  control,
  name,
  label,
  toggleType,
  toggleItems,
  description,
}: Props<TFieldValues>) {
  return (
    <Controller
      control={control}
      name={name}
      render={({ field, fieldState }) => (
        <Field className="grid gap-1">
          <FieldLabel htmlFor={name}>{label}</FieldLabel>
          <ToggleGroup
            id={name}
            type={toggleType}
            value={field.value}
            onValueChange={(v: string | undefined) => v && field.onChange(v)}
            aria-describedby={fieldState.error ? `${name}-msg` : undefined}
            className="flex gap-2"
          >
            {toggleItems.map((c) => (
              <ToggleGroupItem
                key={c.value}
                value={c.value}
                className={cn(
                  "flex items-center gap-2 px-3 py-2 border rounded-md transition-colors",
                  c.on,
                  "data-[state=on]:border-transparent",
                )}
              >
                {c.dot && (
                  <span
                    aria-hidden
                    className={cn("h-3 w-3 rounded-full", c.dot)}
                  />
                )}
                {c.label}
              </ToggleGroupItem>
            ))}
          </ToggleGroup>

          {description && <FieldDescription>{description}</FieldDescription>}
          {fieldState.error?.message && (
            <FieldError>{fieldState.error.message}</FieldError>
          )}
        </Field>
      )}
    />
  );
}
