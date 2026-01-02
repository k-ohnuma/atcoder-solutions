"use client";

import * as React from "react";
import { type Control, FieldPath, FieldValues } from "react-hook-form";
import { Input } from "@/components/ui/input";
import { FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from "@/components/ui/form";

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
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem className="grid gap-2">
          <FormLabel>{label}</FormLabel>
          <FormControl>
            <Input type={type} placeholder={placeholder} {...field} />
          </FormControl>
          {description && <FormDescription>{description}</FormDescription>}
          <FormMessage />
        </FormItem>
      )}
    />
  );
}
