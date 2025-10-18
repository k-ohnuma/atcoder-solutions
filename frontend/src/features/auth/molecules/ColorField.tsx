"use client";

import { FieldPath, type Control, type FieldValues } from "react-hook-form";
import { RhfToggleField } from "../atoms/RhfToggleField";

const toggleItems = [
  {
    value: "red",
    label: "赤",
    dot: "bg-red-500",
    on: "data-[state=on]:bg-red-500/60 data-[state=on]:text-white",
  },
  {
    value: "orange",
    label: "橙",
    dot: "bg-orange-400",
    on: "data-[state=on]:bg-orange-500/60 data-[state=on]:text-white",
  },
  {
    value: "yellow",
    label: "黄",
    dot: "bg-yellow-400",
    on: "data-[state=on]:bg-yellow-500/60 data-[state=on]:text-white",
  },
  {
    value: "blue",
    label: "青",
    dot: "bg-blue-500",
    on: "data-[state=on]:bg-blue-500/60 data-[state=on]:text-white",
  },
  {
    value: "cyan",
    label: "水",
    dot: "bg-cyan-500",
    on: "data-[state=on]:bg-cyan-500/60 data-[state=on]:text-white",
  },
  {
    value: "green",
    label: "緑",
    dot: "bg-green-500",
    on: "data-[state=on]:bg-green-500/60 data-[state=on]:text-white",
  },
  {
    value: "brown",
    label: "茶",
    dot: "bg-[#8b5e3c]",
    on: "data-[state=on]:bg-[#8b5e3c]/60 data-[state=on]:text-white",
  },
  {
    value: "gray",
    label: "灰",
    dot: "bg-gray-500",
    on: "data-[state=on]:bg-gray-500/60 data-[state=on]:text-white",
  },
];
export function ColorField<T extends FieldValues>({
  control,
}: {
  control: Control<T>;
}) {
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
