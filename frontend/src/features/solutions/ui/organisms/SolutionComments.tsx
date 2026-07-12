"use client";

import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from "@/components/ui/accordion";
import { useSolutionComments } from "@/features/solutions/hooks/useSolutionComments";
import { SolutionComment } from "@/shared/model/solution";
import { CommentForm } from "../molecules/CommentForm";
import { DeleteCommentDialog } from "../molecules/DeleteCommentDialog";
import { CommentList } from "./CommentList";

type SolutionCommentsProps = {
  solutionId: string;
  initialComments: SolutionComment[];
};

export function SolutionComments({ solutionId, initialComments }: SolutionCommentsProps) {
  const {
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
    deletePendingComment,
  } = useSolutionComments({ solutionId, initialComments });

  return (
    <section className="space-y-4">
      <h2 className="text-xl font-semibold">コメント</h2>
      <Accordion type="multiple" className="px-1">
        <AccordionItem value="comments">
          <AccordionTrigger>コメント一覧 ({comments.length})</AccordionTrigger>
          <AccordionContent>
            <CommentList
              comments={comments}
              currentUserId={currentUserId}
              editingCommentId={editingCommentId}
              editingBodyMd={editingBodyMd}
              editErrorMessage={editErrorMessage}
              isSubmitting={isSubmitting}
              onStartEditAction={startEditComment}
              onEditBodyChangeAction={setEditingBodyMd}
              onUpdateCommentAction={submitEditComment}
              onCancelEditAction={cancelEditComment}
              onRequestDeleteAction={requestDeleteComment}
            />
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="comment-form">
          <AccordionTrigger>コメントを書く</AccordionTrigger>
          <AccordionContent>
            <CommentForm
              bodyMd={bodyMd}
              errorMessage={error}
              isSubmitting={isSubmitting}
              onBodyChangeAction={setBodyMd}
              onSubmitAction={submitComment}
            />
          </AccordionContent>
        </AccordionItem>
      </Accordion>
      <DeleteCommentDialog
        open={isDeleteDialogOpen}
        onOpenChangeAction={closeDeleteDialog}
        isSubmitting={isSubmitting}
        canDelete={!!pendingDeleteCommentId}
        onDeleteAction={deletePendingComment}
      />
    </section>
  );
}
