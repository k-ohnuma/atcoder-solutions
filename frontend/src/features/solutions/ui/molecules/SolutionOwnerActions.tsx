"use client";

import { Button } from "@/components/ui/button";
import { useSolutionOwnerActions } from "@/features/solutions/hooks/useSolutionOwnerActions";
import { DeleteSolutionDialog } from "./DeleteSolutionDialog";
import { SolutionEditForm } from "./SolutionEditForm";

type SolutionOwnerActionsProps = {
  solutionId: string;
  problemId: string;
  ownerUserId: string;
  title: string;
  bodyMd: string;
  submitUrl: string;
  tags: string[];
};

export function SolutionOwnerActions({
  solutionId,
  problemId,
  ownerUserId,
  title,
  bodyMd,
  submitUrl,
  tags,
}: SolutionOwnerActionsProps) {
  const { form, isOwner, editing, setEditing, isSubmitting, error, isDeleteDialogOpen, setIsDeleteDialogOpen, save, remove } =
    useSolutionOwnerActions({
      solutionId,
      problemId,
      ownerUserId,
      title,
      bodyMd,
      submitUrl,
      tags,
    });

  if (!isOwner) {
    return null;
  }

  return (
    <section className="rounded-xl bg-card p-3">
      <div className="flex flex-wrap items-center gap-2">
        <Button type="button" size="sm" variant="outline" onClick={() => setEditing((v) => !v)} disabled={isSubmitting}>
          {editing ? "編集を閉じる" : "解説を編集"}
        </Button>
        <Button
          type="button"
          size="sm"
          variant="destructive"
          onClick={() => setIsDeleteDialogOpen(true)}
          disabled={isSubmitting}
        >
          解説を削除
        </Button>
      </div>

      {editing && <SolutionEditForm form={form} isSubmitting={isSubmitting} onSubmitAction={save} />}
      {error && <p className="mt-2 text-sm text-destructive">{error}</p>}
      <DeleteSolutionDialog
        open={isDeleteDialogOpen}
        onOpenChangeAction={setIsDeleteDialogOpen}
        isSubmitting={isSubmitting}
        onDeleteAction={remove}
      />
    </section>
  );
}
