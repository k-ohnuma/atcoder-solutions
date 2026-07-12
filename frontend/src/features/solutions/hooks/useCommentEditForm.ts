"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { CommentFormInput, commentFormSchema } from "@/features/solutions/model/comment";
import { SolutionComment } from "@/shared/model/solution";

type UseCommentEditFormParams = {
  onSubmit: (commentId: string, values: CommentFormInput) => Promise<void>;
};

export function useCommentEditForm({ onSubmit }: UseCommentEditFormParams) {
  const [editingCommentId, setEditingCommentId] = useState<string | null>(null);
  const editForm = useForm<CommentFormInput>({
    resolver: zodResolver(commentFormSchema),
    defaultValues: {
      bodyMd: "",
    },
    mode: "onSubmit",
  });
  const {
    handleSubmit,
    watch,
    setValue,
    reset,
    formState: { errors },
  } = editForm;

  const startEditComment = (comment: SolutionComment) => {
    setEditingCommentId(comment.id);
    reset({ bodyMd: comment.bodyMd });
  };

  const cancelEditComment = () => {
    setEditingCommentId(null);
    reset({ bodyMd: "" });
  };

  const finishEditComment = () => {
    setEditingCommentId(null);
    reset({ bodyMd: "" });
  };

  return {
    editingCommentId,
    editingBodyMd: watch("bodyMd"),
    editErrorMessage: errors.bodyMd?.message,
    startEditComment,
    setEditingBodyMd: (value: string) => setValue("bodyMd", value),
    submitEditComment: (commentId: string) => handleSubmit((values) => onSubmit(commentId, values))(),
    cancelEditComment,
    finishEditComment,
  };
}
