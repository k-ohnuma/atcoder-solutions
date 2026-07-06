"use client";

import { Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";

type ShowMoreButtonProps = {
  isLoading: boolean;
  onClickAction: () => void;
};

export function ShowMoreButton({ isLoading, onClickAction }: ShowMoreButtonProps) {
  return (
    <Button variant="outline" disabled={isLoading} onClick={onClickAction}>
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
