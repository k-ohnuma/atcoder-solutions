"use client";

import * as React from "react";
import { Check, ChevronsUpDown } from "lucide-react";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem } from "@/components/ui/command";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover";
import { Contest } from "@/server/domain/contests";

type Props = {
  items: Contest[];
  value: string | null;
  onChangeAction: (id: string | null) => void;
  isLoading?: boolean;
  disabled?: boolean;
  placeholder?: string;
};

export function ContestCombobox({
  items,
  value,
  onChangeAction,
  isLoading,
  disabled,
  placeholder = "コンテストを選択",
}: Props) {
  const [open, setOpen] = React.useState(false);

  const selected = React.useMemo(() => items.find((x) => x.code === value) ?? null, [items, value]);

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          type="button"
          variant="outline"
          role="combobox"
          aria-expanded={open}
          className="w-full justify-between"
          disabled={disabled}
        >
          <span className={cn("truncate", !selected && "text-muted-foreground")}>
            {isLoading ? "読み込み中..." : selected ? `${selected.code.toUpperCase()}` : placeholder}
          </span>
          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-60" />
        </Button>
      </PopoverTrigger>

      <PopoverContent className="w-[--radix-popover-trigger-width] p-0" align="start">
        <Command>
          <CommandInput placeholder="検索（例: abc333 / 333）" />
          <CommandEmpty>見つかりませんでした</CommandEmpty>

          <CommandGroup className="max-h-72 overflow-y-auto">
            <CommandItem
              value="__clear__"
              onSelect={() => {
                onChangeAction(null);
                setOpen(false);
              }}
              className="text-muted-foreground"
            >
              クリア
            </CommandItem>

            {items.map((c) => (
              <CommandItem
                key={c.code}
                value={`${c.code}`}
                onSelect={() => {
                  onChangeAction(c.code);
                  setOpen(false);
                }}
              >
                <Check className={cn("mr-2 h-4 w-4", value === c.code ? "opacity-100" : "opacity-0")} />
                <div className="min-w-0">
                  <div className="truncate text-sm font-medium">{c.code.toUpperCase()}</div>
                </div>
              </CommandItem>
            ))}
          </CommandGroup>
        </Command>
      </PopoverContent>
    </Popover>
  );
}
