"use client";

import { FieldPath, type Control, type FieldValues } from "react-hook-form";
import { RhfToggleField } from "../atoms/RhfToggleField";

const toggleItems = [
  {
    value: "red",
    label: "赤",
  },
  {
    value: "orange",
    label: "橙",
  },
  {
    value: "yellow",
    label: "黄",
  },
  {
    value: "blue",
    label: "青",
  },
  {
    value: "cyan",
    label: "水",
  },
  {
    value: "green",
    label: "緑",
  },
  {
    value: "brown",
    label: "茶",
  },
  {
    value: "gray",
    label: "灰",
  },
];
export function ColorField<T extends FieldValues>({ control }: { control: Control<T> }) {
  return (
    <RhfToggleField
      control={control}
      name={"color" as FieldPath<T>}
      label="色"
      toggleType="single"
      toggleItems={toggleItems}
      description="あなたは何色にでもなれます"
    />
  );
}
