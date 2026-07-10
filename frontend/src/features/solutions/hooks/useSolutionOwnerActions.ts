"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { onAuthStateChanged } from "firebase/auth";
import { useRouter } from "next/navigation";
import { useEffect, useMemo, useState } from "react";
import { useForm } from "react-hook-form";
import { useToast } from "@/components/ui/toast";
import { solutionApi } from "@/features/solutions/api/solutionApi";
import { UpdateSolutionFormInput, updateSolutionFormSchema } from "@/features/solutions/model/updateSolution";
import { getFirebaseIdToken } from "@/lib/client/firebaseToken";
import { getFirebaseAuth } from "@/shared/firebase/client";

type UseSolutionOwnerActionsParams = {
  solutionId: string;
  problemId: string;
  ownerUserId: string;
  title: string;
  bodyMd: string;
  submitUrl: string;
  tags: string[];
};

export function useSolutionOwnerActions({
  solutionId,
  problemId,
  ownerUserId,
  title,
  bodyMd,
  submitUrl,
  tags,
}: UseSolutionOwnerActionsParams) {
  const router = useRouter();
  const auth = getFirebaseAuth();
  const { toast } = useToast();
  const [currentUserId, setCurrentUserId] = useState<string | null>(null);
  const [editing, setEditing] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);
  const form = useForm<UpdateSolutionFormInput>({
    resolver: zodResolver(updateSolutionFormSchema),
    defaultValues: {
      title,
      bodyMd,
      submitUrl,
      tags,
    },
    mode: "onSubmit",
  });
  const { reset } = form;

  useEffect(() => {
    const unsub = onAuthStateChanged(auth, (user) => {
      setCurrentUserId(user?.uid ?? null);
    });
    return () => unsub();
  }, [auth]);

  useEffect(() => {
    reset({
      title,
      bodyMd,
      submitUrl,
      tags,
    });
  }, [bodyMd, reset, submitUrl, tags, title]);

  const isOwner = useMemo(() => currentUserId !== null && currentUserId === ownerUserId, [currentUserId, ownerUserId]);

  const save = async (values: UpdateSolutionFormInput) => {
    if (isSubmitting) {
      return;
    }
    const token = await getFirebaseIdToken();
    if (!token) {
      router.push("/signin");
      return;
    }
    setIsSubmitting(true);
    setError(null);
    try {
      const normalizedTags = values.tags.map((v) => v.trim()).filter((v) => v.length > 0);
      await solutionApi.update(solutionId, values.title, values.bodyMd, values.submitUrl, normalizedTags, token);
      toast({ title: "解説を更新しました" });
      setEditing(false);
      router.refresh();
    } catch (error) {
      const message = error instanceof Error ? error.message : "解説の更新に失敗しました。";
      setError(message);
      toast({ title: "解説の更新に失敗しました", description: message, variant: "error" });
    } finally {
      setIsSubmitting(false);
    }
  };

  const remove = async () => {
    if (isSubmitting) {
      return;
    }
    const token = await getFirebaseIdToken();
    if (!token) {
      router.push("/signin");
      return;
    }
    setIsSubmitting(true);
    setError(null);
    try {
      await solutionApi.delete(solutionId, token);
      toast({ title: "解説を削除しました" });
      setIsDeleteDialogOpen(false);
      router.push(`/problems/${problemId}`);
    } catch (error) {
      const message = error instanceof Error ? error.message : "解説の削除に失敗しました。";
      setError(message);
      toast({ title: "解説の削除に失敗しました", description: message, variant: "error" });
    } finally {
      setIsSubmitting(false);
    }
  };

  return {
    form,
    isOwner,
    editing,
    setEditing,
    isSubmitting,
    error,
    isDeleteDialogOpen,
    setIsDeleteDialogOpen,
    save,
    remove,
  };
}
