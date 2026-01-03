"use client";

import { Label } from "@radix-ui/react-label";
import React from "react";
import { cn } from "@/lib/utils";

type TextInputProps = {
  value: string;
  onChangeAction: (value: string) => void;
  placeholder?: string;
  className?: string;
};

export const TextInput: React.FC<TextInputProps> = ({ value, onChangeAction, placeholder, className }) => {
  return (
    <div>
      <Label className="text-sm mb-3">{placeholder}</Label>
      <input
        className={cn(
          "h-10 rounded-lg border border-border bg-background px-3 text-sm outline-none",
          "focus-visible:ring-1 focus-visible:ring-ring",
          className,
        )}
        value={value}
        placeholder={placeholder}
        onChange={(e) => onChangeAction(e.target.value)}
      />
    </div>
  );
};
