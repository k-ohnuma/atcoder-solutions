"use client";

import { onAuthStateChanged } from "firebase/auth";
import { useEffect, useState } from "react";
import { useToast } from "@/components/ui/toast";
import { solutionApi } from "@/features/solutions/api/solutionApi";
import { useAuthenticatedSolutionMutation } from "@/features/solutions/hooks/useAuthenticatedSolutionMutation";
import { getFirebaseAuth } from "@/shared/firebase/client";

type UseSolutionLikeParams = {
  solutionId: string;
  initialVotesCount: number;
};

export function useSolutionLike({ solutionId, initialVotesCount }: UseSolutionLikeParams) {
  const { toast } = useToast();
  const { isSubmitting, runMutation } = useAuthenticatedSolutionMutation();
  const auth = getFirebaseAuth();
  const [liked, setLiked] = useState(false);
  const [votesCount, setVotesCount] = useState(initialVotesCount);
  const [isLikeStatusReady, setIsLikeStatusReady] = useState(false);

  useEffect(() => {
    setVotesCount(initialVotesCount);
  }, [initialVotesCount]);

  useEffect(() => {
    setIsLikeStatusReady(false);
    const unsub = onAuthStateChanged(auth, async (user) => {
      if (!user) {
        setLiked(false);
        setIsLikeStatusReady(true);
        return;
      }

      try {
        const token = await user.getIdToken();
        const isLiked = await solutionApi.getMyLikeStatus(solutionId, token);
        setLiked(isLiked);
      } catch (error) {
        const message = error instanceof Error ? error.message : "いいね状態の取得に失敗しました。";
        setLiked(false);
        toast({ title: "いいね状態の取得に失敗しました", description: message, variant: "error" });
      } finally {
        setIsLikeStatusReady(true);
      }
    });

    return () => {
      unsub();
    };
  }, [auth, solutionId, toast]);

  const toggleLike = async () => {
    const nextLiked = await runMutation({
      action: (token) => (liked ? solutionApi.unvote(solutionId, token) : solutionApi.vote(solutionId, token)),
      fallbackErrorMessage: "いいねの更新に失敗しました。",
      errorToastTitle: "いいねの更新に失敗しました",
    });
    if (nextLiked === null) {
      return;
    }

    setLiked(nextLiked);
    if (liked && !nextLiked) {
      setVotesCount((prev) => Math.max(0, prev - 1));
      return;
    }
    if (!liked && nextLiked) {
      setVotesCount((prev) => prev + 1);
    }
  };

  return {
    liked,
    votesCount,
    isDisabled: isSubmitting || !isLikeStatusReady,
    toggleLike,
  };
}
