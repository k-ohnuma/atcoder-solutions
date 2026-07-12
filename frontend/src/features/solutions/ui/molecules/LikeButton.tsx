"use client";

import { Heart } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useSolutionLike } from "@/features/solutions/hooks/useSolutionLike";

type LikeButtonProps = {
  solutionId: string;
  initialVotesCount: number;
};

export function LikeButton({ solutionId, initialVotesCount }: LikeButtonProps) {
  const { liked, votesCount, isDisabled, toggleLike } = useSolutionLike({ solutionId, initialVotesCount });

  return (
    <Button
      type="button"
      variant={liked ? "default" : "outline"}
      size="sm"
      disabled={isDisabled}
      onClick={toggleLike}
      className="gap-1.5"
      aria-label={liked ? `いいね済み (${votesCount})` : `いいね (${votesCount})`}
      title={liked ? "いいね済み" : "いいね"}
    >
      <Heart className={`size-4 ${liked ? "fill-current" : ""}`} />
      <span>{votesCount}</span>
    </Button>
  );
}
