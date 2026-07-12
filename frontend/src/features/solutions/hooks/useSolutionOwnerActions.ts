"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useRouter } from "next/navigation";
import { useEffect, useMemo, useState } from "react";
import { useForm } from "react-hook-form";
import { useToast } from "@/components/ui/toast";
import { useFirebaseUserId } from "@/features/auth/hooks/useFirebaseUserId";
import { solutionApi } from "@/features/solutions/api/solutionApi";
import { useAuthenticatedSolutionMutation } from "@/features/solutions/hooks/useAuthenticatedSolutionMutation";
import { UpdateSolutionFormInput, updateSolutionFormSchema } from "@/features/solutions/model/updateSolution";

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
  const { toast } = useToast();
  const currentUserId = useFirebaseUserId();
  const { isSubmitting, error, runMutation } = useAuthenticatedSolutionMutation();
  const [editing, setEditing] = useState(false);
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
    reset({
      title,
      bodyMd,
      submitUrl,
      tags,
    });
  }, [bodyMd, reset, submitUrl, tags, title]);

  const isOwner = useMemo(() => currentUserId !== null && currentUserId === ownerUserId, [currentUserId, ownerUserId]);

  const save = async (values: UpdateSolutionFormInput) => {
    const normalizedTags = values.tags.map((v) => v.trim()).filter((v) => v.length > 0);
    const updated = await runMutation({
      action: async (token) => {
        await solutionApi.update(solutionId, values.title, values.bodyMd, values.submitUrl, normalizedTags, token);
        return true;
      },
      fallbackErrorMessage: "解説の更新に失敗しました。",
      errorToastTitle: "解説の更新に失敗しました",
    });
    if (!updated) {
      return;
    }

    toast({ title: "解説を更新しました" });
    setEditing(false);
    router.refresh();
  };

  const remove = async () => {
    const deleted = await runMutation({
      action: async (token) => {
        await solutionApi.delete(solutionId, token);
        return true;
      },
      fallbackErrorMessage: "解説の削除に失敗しました。",
      errorToastTitle: "解説の削除に失敗しました",
    });
    if (!deleted) {
      return;
    }

    toast({ title: "解説を削除しました" });
    setIsDeleteDialogOpen(false);
    router.push(`/problems/${problemId}`);
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
