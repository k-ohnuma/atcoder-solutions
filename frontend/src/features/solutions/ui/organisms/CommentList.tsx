import { formatDateTime } from "@/shared/lib/date";
import { SolutionComment } from "@/shared/model/solution";
import { MarkdownRenderer } from "../atoms/MarkdownRenderer";
import { CommentEditForm } from "../molecules/CommentEditForm";

type CommentListProps = {
  comments: SolutionComment[];
  currentUserId: string | null;
  editingCommentId: string | null;
  editingBodyMd: string;
  editErrorMessage?: string;
  isSubmitting: boolean;
  onStartEditAction: (comment: SolutionComment) => void;
  onEditBodyChangeAction: (value: string) => void;
  onUpdateCommentAction: (commentId: string) => void;
  onCancelEditAction: () => void;
  onRequestDeleteAction: (commentId: string) => void;
};

export function CommentList({
  comments,
  currentUserId,
  editingCommentId,
  editingBodyMd,
  editErrorMessage,
  isSubmitting,
  onStartEditAction,
  onEditBodyChangeAction,
  onUpdateCommentAction,
  onCancelEditAction,
  onRequestDeleteAction,
}: CommentListProps) {
  if (comments.length === 0) {
    return <p className="text-sm text-muted-foreground">まだコメントはありません。</p>;
  }

  return (
    <div className="space-y-3">
      {comments.map((comment) => (
        <article key={comment.id} className="rounded-xl bg-muted/30 p-4">
          <div className="mb-3 flex flex-wrap items-center gap-2 text-xs text-muted-foreground">
            <span>{comment.userName}</span>
            <span>{formatDateTime(comment.createdAt)}</span>
            {currentUserId === comment.userId && (
              <span className="ml-auto flex items-center gap-2">
                <button type="button" className="text-xs hover:underline" onClick={() => onStartEditAction(comment)}>
                  編集
                </button>
                <button
                  type="button"
                  className="text-xs text-destructive hover:underline"
                  onClick={() => onRequestDeleteAction(comment.id)}
                >
                  削除
                </button>
              </span>
            )}
          </div>
          {editingCommentId === comment.id ? (
            <CommentEditForm
              bodyMd={editingBodyMd}
              errorMessage={editErrorMessage}
              isSubmitting={isSubmitting}
              onBodyChangeAction={onEditBodyChangeAction}
              onSubmitAction={() => onUpdateCommentAction(comment.id)}
              onCancelAction={onCancelEditAction}
            />
          ) : (
            <MarkdownRenderer value={comment.bodyMd} />
          )}
        </article>
      ))}
    </div>
  );
}
