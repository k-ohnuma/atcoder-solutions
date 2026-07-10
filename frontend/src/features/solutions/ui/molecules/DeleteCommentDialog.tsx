import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@/components/ui/dialog";

type DeleteCommentDialogProps = {
  open: boolean;
  isSubmitting: boolean;
  canDelete: boolean;
  onOpenChangeAction: (open: boolean) => void;
  onDeleteAction: () => void;
};

export function DeleteCommentDialog({
  open,
  isSubmitting,
  canDelete,
  onOpenChangeAction,
  onDeleteAction,
}: DeleteCommentDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChangeAction}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>このコメントを削除しますか？</DialogTitle>
          <DialogDescription>削除すると元に戻せません。</DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button type="button" variant="outline" onClick={() => onOpenChangeAction(false)} disabled={isSubmitting}>
            キャンセル
          </Button>
          <Button type="button" variant="destructive" onClick={onDeleteAction} disabled={isSubmitting || !canDelete}>
            {isSubmitting ? "削除中..." : "削除する"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
