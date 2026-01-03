"use client";

import { type Control, FieldPath, FieldValues } from "react-hook-form";
import { FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from "@/components/ui/form";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";

type ToggleItem = {
  label: string;
  value: string;
};

type Props<TFieldValues extends FieldValues> = {
  control: Control<TFieldValues>;
  name: FieldPath<TFieldValues>;
  label: string;
  toggleItems: ToggleItem[];
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
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem className="grid gap-2">
          <FormLabel>{label}</FormLabel>

          <FormControl>
            <ToggleGroup
              type="single"
              value={field.value}
              onValueChange={(v) => field.onChange(v)}
              className="flex justify-between gap-2"
            >
              {toggleItems.map((c) => (
                <ToggleGroupItem key={c.value} value={c.value} className="gap-2">
                  {c.label}
                </ToggleGroupItem>
              ))}
            </ToggleGroup>
          </FormControl>

          {description && <FormDescription>{description}</FormDescription>}
          <FormMessage />
        </FormItem>
      )}
    />
  );
}
