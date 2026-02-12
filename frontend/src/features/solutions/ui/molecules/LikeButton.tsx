"use client";

import { onAuthStateChanged } from "firebase/auth";
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
  const auth = getFirebaseAuth();

  useEffect(() => {
    const unsub = onAuthStateChanged(auth, async (user) => {
      if (!user) {
        setLiked(false);
        return;
      }

      const token = await user.getIdToken();
      const isLiked = await apiClient.getMySolutionLikeStatus(solutionId, token);
      setLiked(isLiked);
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
    <Button type="button" variant={liked ? "default" : "outline"} size="sm" disabled={isSubmitting} onClick={onClick}>
      {liked ? "いいね済み" : "いいね"} {votesCount}
    </Button>
  );
}
