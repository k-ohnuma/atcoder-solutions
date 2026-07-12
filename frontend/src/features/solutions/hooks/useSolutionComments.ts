"use client";

import { useState } from "react";
import { useToast } from "@/components/ui/toast";
import { useFirebaseUserId } from "@/features/auth/hooks/useFirebaseUserId";
import { solutionApi } from "@/features/solutions/api/solutionApi";
import { useAuthenticatedSolutionMutation } from "@/features/solutions/hooks/useAuthenticatedSolutionMutation";
import { useCommentEditForm } from "@/features/solutions/hooks/useCommentEditForm";
import { CommentFormInput } from "@/features/solutions/model/comment";
import { SolutionComment } from "@/shared/model/solution";

type UseSolutionCommentsParams = {
  solutionId: string;
  initialComments: SolutionComment[];
};

export function useSolutionComments({ solutionId, initialComments }: UseSolutionCommentsParams) {
  const { toast } = useToast();
  const currentUserId = useFirebaseUserId();
  const { isSubmitting, error, setError, runMutation } = useAuthenticatedSolutionMutation();
  const [comments, setComments] = useState(initialComments);
  const [bodyMd, setBodyMd] = useState("");
  const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);
  const [pendingDeleteCommentId, setPendingDeleteCommentId] = useState<string | null>(null);
  const {
    editingCommentId,
    editingBodyMd,
    editErrorMessage,
    startEditComment: startEditForm,
    setEditingBodyMd,
    submitEditComment,
    cancelEditComment,
    finishEditComment,
  } = useCommentEditForm({ onSubmit: updateComment });

  const submitComment = async () => {
    const normalized = bodyMd.trim();
    if (!normalized) {
      setError("コメントを入力してください");
      return;
    }
    if (normalized.length > 2000) {
      setError("コメントは2000文字以内で入力してください");
      return;
    }

    const created = await runMutation({
      action: (token) => solutionApi.createComment(solutionId, normalized, token),
      fallbackErrorMessage: "コメントの投稿に失敗しました",
      errorToastTitle: "コメントの投稿に失敗しました",
    });
    if (!created) {
      return;
    }

    setComments((prev) => [...prev, created]);
    setBodyMd("");
    toast({ title: "コメントを投稿しました" });
  };

  const startEditComment = (comment: SolutionComment) => {
    startEditForm(comment);
    setError(null);
  };

  const requestDeleteComment = (commentId: string) => {
    setPendingDeleteCommentId(commentId);
    setIsDeleteDialogOpen(true);
  };

  const closeDeleteDialog = (open: boolean) => {
    setIsDeleteDialogOpen(open);
    if (!open) {
      setPendingDeleteCommentId(null);
    }
  };

  async function updateComment(commentId: string, values: CommentFormInput) {
    const normalized = values.bodyMd.trim();

    const updated = await runMutation({
      action: (token) => solutionApi.updateComment(commentId, normalized, token),
      fallbackErrorMessage: "コメントの更新に失敗しました",
      errorToastTitle: "コメントの更新に失敗しました",
    });
    if (!updated) {
      return;
    }

    toast({ title: "コメントを更新しました" });
    setComments((prev) => prev.map((comment) => (comment.id === commentId ? updated : comment)));
    finishEditComment();
  }

  const deleteComment = async (commentId: string) => {
    if (!commentId) {
      return;
    }

    const deleted = await runMutation({
      action: async (token) => {
        await solutionApi.deleteComment(commentId, token);
        return true;
      },
      fallbackErrorMessage: "コメントの削除に失敗しました",
      errorToastTitle: "コメントの削除に失敗しました",
    });
    if (!deleted) {
      return;
    }

    toast({ title: "コメントを削除しました" });
    setComments((prev) => prev.filter((comment) => comment.id !== commentId));
    setIsDeleteDialogOpen(false);
    setPendingDeleteCommentId(null);
    if (editingCommentId === commentId) {
      finishEditComment();
    }
  };

  return {
    comments,
    bodyMd,
    setBodyMd,
    isSubmitting,
    error,
    currentUserId,
    editingCommentId,
    editingBodyMd,
    editErrorMessage,
    isDeleteDialogOpen,
    pendingDeleteCommentId,
    submitComment,
    startEditComment,
    setEditingBodyMd,
    submitEditComment,
    cancelEditComment,
    requestDeleteComment,
    closeDeleteDialog,
    deletePendingComment: () => pendingDeleteCommentId && deleteComment(pendingDeleteCommentId),
  };
}
