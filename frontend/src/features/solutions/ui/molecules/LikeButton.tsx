"use client";

import { onAuthStateChanged } from "firebase/auth";
import { Heart } from "lucide-react";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { ApiClient } from "@/lib/client/apiClient";
import { getFirebaseIdToken } from "@/lib/client/firebaseToken";
import { getFirebaseAuth } from "@/shared/firebase/client";

const apiClient = new ApiClient();

type LikeButtonProps = {
  solutionId: string;
  initialVotesCount: number;
};

export function LikeButton({ solutionId, initialVotesCount }: LikeButtonProps) {
  const router = useRouter();
  const [liked, setLiked] = useState(false);
  const [votesCount, setVotesCount] = useState(initialVotesCount);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isLikeStatusReady, setIsLikeStatusReady] = useState(false);
  const auth = getFirebaseAuth();

  useEffect(() => {
    setIsLikeStatusReady(false);
    const unsub = onAuthStateChanged(auth, async (user) => {
      if (!user) {
        setLiked(false);
        setIsLikeStatusReady(true);
        return;
      }

      const token = await user.getIdToken();
      const isLiked = await apiClient.getMySolutionLikeStatus(solutionId, token);
      setLiked(isLiked);
      setIsLikeStatusReady(true);
    });

    return () => {
      unsub();
    };
  }, [auth, solutionId]);

  const onClick = async () => {
    if (isSubmitting) {
      return;
    }

    const token = await getFirebaseIdToken();
    if (!token) {
      router.push("/signin");
      return;
    }

    setIsSubmitting(true);
    try {
      if (liked) {
        const unliked = await apiClient.unvoteSolution(solutionId, token);
        if (unliked === false) {
          setLiked(false);
          setVotesCount((prev) => Math.max(0, prev - 1));
        }
        return;
      }

      const nextLiked = await apiClient.voteSolution(solutionId, token);
      if (nextLiked) {
        setLiked(true);
        setVotesCount((prev) => prev + 1);
      }
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <Button
      type="button"
      variant={liked ? "default" : "outline"}
      size="sm"
      disabled={isSubmitting || !isLikeStatusReady}
      onClick={onClick}
      className="gap-1.5"
      aria-label={liked ? `いいね済み (${votesCount})` : `いいね (${votesCount})`}
      title={liked ? "いいね済み" : "いいね"}
    >
      <Heart className={`size-4 ${liked ? "fill-current" : ""}`} />
      <span>{votesCount}</span>
    </Button>
  );
}
