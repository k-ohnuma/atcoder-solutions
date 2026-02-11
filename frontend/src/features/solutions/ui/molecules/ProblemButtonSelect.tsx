"use client";

import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import type { Problem } from "@/shared/model/problem";

type ProblemButtonSelectProps = {
  items: Problem[];
  value: string | null;
  onChangeAction: (id: string) => void;
  disabled?: boolean;
  isLoading?: boolean;
};

export function ProblemButtonSelect({ items, value, onChangeAction, disabled, isLoading }: ProblemButtonSelectProps) {
  if (isLoading) {
    return <div className="text-sm text-muted-foreground">問題を読み込み中...</div>;
  }

  return (
    <div className={cn("grid grid-cols-2 gap-2 sm:grid-cols-3", disabled && "opacity-60")}>
      {items.map((problem) => {
        const selected = problem.id === value;
        return (
          <Button
            key={problem.id}
            type="button"
            variant={selected ? "default" : "outline"}
            className="justify-start"
            disabled={disabled}
            onClick={() => onChangeAction(problem.id)}
          >
            <span className="mr-2 text-xs">{problem.problemIndex}</span>
            <span className="truncate">{problem.title}</span>
          </Button>
        );
      })}
    </div>
  );
}
