"use client";

import { FieldErrors } from "react-hook-form";
import { CreateSolutionInput } from "@/shared/model/solutionCreate";
import { TextInput } from "../atoms/TextInput";
import { TagsInputField } from "./TagInput";

type CreateSolutionMetadataFieldsProps = {
  title: string;
  tags: string[];
  submitUrl: string;
  errors: FieldErrors<CreateSolutionInput>;
  onTitleChangeAction: (value: string) => void;
  onTagsChangeAction: (value: string[]) => void;
  onSubmitUrlChangeAction: (value: string) => void;
};

export function CreateSolutionMetadataFields({
  title,
  tags,
  submitUrl,
  errors,
  onTitleChangeAction,
  onTagsChangeAction,
  onSubmitUrlChangeAction,
}: CreateSolutionMetadataFieldsProps) {
  return (
    <div className="grid gap-3 md:grid-cols-2">
      <div className="space-y-1 md:col-span-2">
        <TextInput value={title} onChangeAction={onTitleChangeAction} placeholder="タイトル" className="w-full" />
        {errors.title && <p className="text-xs text-destructive">{errors.title.message}</p>}
      </div>
      <div className="space-y-1">
        <TagsInputField values={tags} onChangeAction={onTagsChangeAction} label="タグ" className="w-full" />
        {errors.tags && <p className="text-xs text-destructive">{errors.tags.message}</p>}
      </div>
      <div className="space-y-1">
        <TextInput
          value={submitUrl}
          onChangeAction={onSubmitUrlChangeAction}
          placeholder="提出URL（任意）"
          className="w-full"
        />
        {errors.submitUrl && <p className="text-xs text-destructive">{errors.submitUrl.message}</p>}
      </div>
    </div>
  );
}
