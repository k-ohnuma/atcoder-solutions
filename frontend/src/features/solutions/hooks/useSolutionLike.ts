"use client";

import { onAuthStateChanged } from "firebase/auth";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { useToast } from "@/components/ui/toast";
import { solutionApi } from "@/features/solutions/api/solutionApi";
import { getFirebaseIdToken } from "@/lib/client/firebaseToken";
import { getFirebaseAuth } from "@/shared/firebase/client";

type UseSolutionLikeParams = {
  solutionId: string;
  initialVotesCount: number;
};

export function useSolutionLike({ solutionId, initialVotesCount }: UseSolutionLikeParams) {
  const router = useRouter();
  const { toast } = useToast();
  const auth = getFirebaseAuth();
  const [liked, setLiked] = useState(false);
  const [votesCount, setVotesCount] = useState(initialVotesCount);
  const [isSubmitting, setIsSubmitting] = useState(false);
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
        const nextLiked = await solutionApi.unvote(solutionId, token);
        setLiked(nextLiked);
        if (!nextLiked) {
          setVotesCount((prev) => Math.max(0, prev - 1));
        }
        return;
      }

      const nextLiked = await solutionApi.vote(solutionId, token);
      setLiked(nextLiked);
      if (nextLiked) {
        setVotesCount((prev) => prev + 1);
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : "いいねの更新に失敗しました。";
      toast({ title: "いいねの更新に失敗しました", description: message, variant: "error" });
    } finally {
      setIsSubmitting(false);
    }
  };

  return {
    liked,
    votesCount,
    isDisabled: isSubmitting || !isLikeStatusReady,
    toggleLike,
  };
}
