"use client";

import { Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";

type ShowAllButtonProps = {
  isLoading: boolean;
  onClick: () => void;
};

export function ShowAllButton({ isLoading, onClick }: ShowAllButtonProps) {
  return (
    <Button variant="outline" disabled={isLoading} onClick={onClick}>
      {isLoading ? (
        <>
          <Loader2 className="size-4 animate-spin" />
          読み込み中...
        </>
      ) : (
        <>さらに読み込む</>
      )}
    </Button>
  );
}
