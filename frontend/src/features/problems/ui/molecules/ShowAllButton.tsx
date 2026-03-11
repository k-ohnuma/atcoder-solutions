"use client";

import { Loader2 } from "lucide-react";
import { useRouter } from "next/navigation";
import { useTransition } from "react";
import { Button } from "@/components/ui/button";

type ShowAllButtonProps = {
  href: string;
  remainingContestCount: number;
};

export function ShowAllButton({ href, remainingContestCount }: ShowAllButtonProps) {
  const router = useRouter();
  const [isPending, startTransition] = useTransition();

  return (
    <Button
      variant="outline"
      disabled={isPending}
      onClick={() => {
        startTransition(() => {
          router.push(href, { scroll: false });
        });
      }}
    >
      {isPending ? (
        <>
          <Loader2 className="size-4 animate-spin" />
          読み込み中...
        </>
      ) : (
        <>すべて表示（残り{remainingContestCount}件）</>
      )}
    </Button>
  );
}
