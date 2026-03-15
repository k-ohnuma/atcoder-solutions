"use client";

import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@/components/ui/dialog";

type DeleteAccountDialogsProps = {
  isDeleteDialogOpen: boolean;
  onDeleteDialogOpenChangeAction: (open: boolean) => void;
  isDeleteErrorDialogOpen: boolean;
  onDeleteErrorDialogOpenChangeAction: (open: boolean) => void;
  deleteErrorMessage: string;
  isDeleting: boolean;
  onDeleteAccountAction: () => void;
};

export function DeleteAccountDialogs({
  isDeleteDialogOpen,
  onDeleteDialogOpenChangeAction,
  isDeleteErrorDialogOpen,
  onDeleteErrorDialogOpenChangeAction,
  deleteErrorMessage,
  isDeleting,
  onDeleteAccountAction,
}: DeleteAccountDialogsProps) {
  return (
    <>
      <Dialog open={isDeleteDialogOpen} onOpenChange={onDeleteDialogOpenChangeAction}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>アカウントを削除しますか？</DialogTitle>
            <DialogDescription>この操作は取り消せません。</DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button type="button" variant="outline" onClick={() => onDeleteDialogOpenChangeAction(false)} disabled={isDeleting}>
              キャンセル
            </Button>
            <Button type="button" variant="destructive" onClick={onDeleteAccountAction} disabled={isDeleting}>
              {isDeleting ? "削除中..." : "削除する"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
      <Dialog open={isDeleteErrorDialogOpen} onOpenChange={onDeleteErrorDialogOpenChangeAction}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>アカウント削除に失敗しました</DialogTitle>
            <DialogDescription>{deleteErrorMessage}</DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button type="button" onClick={() => onDeleteErrorDialogOpenChangeAction(false)}>
              閉じる
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
