"use client";

import {
  Controller,
  type Control,
  FieldPath,
  FieldValues,
} from "react-hook-form";
import { Field } from "@/components/ui/field";
import { ToggleGroup } from "@/components/ui/toggle-group";
import { fieldRootStyle } from "./style";
import { css } from "styled-system/css";
import { dotClass, toggleItemClass } from "../molecules/style/colorRecipe";

type ToggleItem = {
  label: string;
  value: string;
  dotColor?: string;
  onColor?: string;
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
  toggleItems,
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
            <ToggleGroup.Root
              id={name}
              value={field.value}
              onValueChange={(v) => {
                field.onChange(v.value[0]);
              }}
              aria-describedby={fieldState.error ? `${name}-msg` : undefined}
              className={css({
                display: "flex",
                gap: "2",
                justifyContent: "space-between",
                border: "none",
              })}
            >
              {toggleItems.map((c) => {
                return (
                  <ToggleGroup.Item
                    key={c.value}
                    value={c.value}
                    className={toggleItemClass({color: c.value as any})}
                  >
                    {c.value && (
                      <span
                        aria-hidden
                        className={dotClass({ color: c.value as any })}
                      />
                    )}
                    {c.label}
                  </ToggleGroup.Item>
                );
              })}
            </ToggleGroup.Root>

            {description && <Field.HelperText>{description}</Field.HelperText>}
            {fieldState.error?.message && (
              <Field.ErrorText>{fieldState.error.message}</Field.ErrorText>
            )}
          </Field.Root>
        );
      }}
    />
  );
}
