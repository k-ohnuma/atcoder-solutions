"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { onAuthStateChanged } from "firebase/auth";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { useToast } from "@/components/ui/toast";
import { solutionApi } from "@/features/solutions/api/solutionApi";
import { CommentFormInput, commentFormSchema } from "@/features/solutions/model/comment";
import { getFirebaseIdToken } from "@/lib/client/firebaseToken";
import { getFirebaseAuth } from "@/shared/firebase/client";
import { SolutionComment } from "@/shared/model/solution";

type UseSolutionCommentsParams = {
  solutionId: string;
  initialComments: SolutionComment[];
};

export function useSolutionComments({ solutionId, initialComments }: UseSolutionCommentsParams) {
  const router = useRouter();
  const auth = getFirebaseAuth();
  const { toast } = useToast();
  const [comments, setComments] = useState(initialComments);
  const [bodyMd, setBodyMd] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [currentUserId, setCurrentUserId] = useState<string | null>(null);
  const [editingCommentId, setEditingCommentId] = useState<string | null>(null);
  const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);
  const [pendingDeleteCommentId, setPendingDeleteCommentId] = useState<string | null>(null);
  const editForm = useForm<CommentFormInput>({
    resolver: zodResolver(commentFormSchema),
    defaultValues: {
      bodyMd: "",
    },
    mode: "onSubmit",
  });
  const {
    handleSubmit: handleEditSubmit,
    watch: watchEdit,
    setValue: setEditValue,
    reset: resetEdit,
    formState: { errors: editErrors },
  } = editForm;
  const editingBodyMd = watchEdit("bodyMd");

  useEffect(() => {
    const unsub = onAuthStateChanged(auth, (user) => {
      setCurrentUserId(user?.uid ?? null);
    });
    return () => unsub();
  }, [auth]);

  const submitComment = async () => {
    if (isSubmitting) {
      return;
    }
    const normalized = bodyMd.trim();
    if (!normalized) {
      setError("コメントを入力してください");
      return;
    }
    if (normalized.length > 2000) {
      setError("コメントは2000文字以内で入力してください");
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
      const created = await solutionApi.createComment(solutionId, normalized, token);
      setComments((prev) => [...prev, created]);
      setBodyMd("");
      toast({ title: "コメントを投稿しました" });
    } catch (error) {
      const message = error instanceof Error ? error.message : "コメントの投稿に失敗しました";
      setError(message);
      toast({ title: "コメントの投稿に失敗しました", description: message, variant: "error" });
    } finally {
      setIsSubmitting(false);
    }
  };

  const startEditComment = (comment: SolutionComment) => {
    setEditingCommentId(comment.id);
    resetEdit({ bodyMd: comment.bodyMd });
    setError(null);
  };

  const cancelEditComment = () => {
    setEditingCommentId(null);
    resetEdit({ bodyMd: "" });
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

  const updateComment = async (commentId: string, values: CommentFormInput) => {
    if (isSubmitting) {
      return;
    }
    const normalized = values.bodyMd.trim();

    const token = await getFirebaseIdToken();
    if (!token) {
      router.push("/signin");
      return;
    }

    setIsSubmitting(true);
    setError(null);
    try {
      const updated = await solutionApi.updateComment(commentId, normalized, token);
      toast({ title: "コメントを更新しました" });
      setComments((prev) => prev.map((comment) => (comment.id === commentId ? updated : comment)));
      setEditingCommentId(null);
      resetEdit({ bodyMd: "" });
    } catch (error) {
      const message = error instanceof Error ? error.message : "コメントの更新に失敗しました";
      setError(message);
      toast({ title: "コメントの更新に失敗しました", description: message, variant: "error" });
    } finally {
      setIsSubmitting(false);
    }
  };

  const deleteComment = async (commentId: string) => {
    if (isSubmitting) {
      return;
    }
    if (!commentId) {
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
      await solutionApi.deleteComment(commentId, token);
      toast({ title: "コメントを削除しました" });
      setComments((prev) => prev.filter((comment) => comment.id !== commentId));
      setIsDeleteDialogOpen(false);
      setPendingDeleteCommentId(null);
      if (editingCommentId === commentId) {
        setEditingCommentId(null);
        resetEdit({ bodyMd: "" });
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : "コメントの削除に失敗しました";
      setError(message);
      toast({ title: "コメントの削除に失敗しました", description: message, variant: "error" });
    } finally {
      setIsSubmitting(false);
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
    editErrorMessage: editErrors.bodyMd?.message,
    isDeleteDialogOpen,
    pendingDeleteCommentId,
    submitComment,
    startEditComment,
    setEditingBodyMd: (value: string) => setEditValue("bodyMd", value),
    submitEditComment: (commentId: string) => handleEditSubmit((values) => updateComment(commentId, values))(),
    cancelEditComment,
    requestDeleteComment,
    closeDeleteDialog,
    deletePendingComment: () => pendingDeleteCommentId && deleteComment(pendingDeleteCommentId),
  };
}
