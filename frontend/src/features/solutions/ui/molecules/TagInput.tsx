"use client";

import { TagsInput, TagsInputInput, TagsInputItem, TagsInputLabel, TagsInputList } from "@/components/ui/tags-input";

type TagsInputFieldProps = {
  label: string
  values: string[];
  onChangeAction: (value: string[]) => void;
  className?: string
};

export function TagsInputField({ values, onChangeAction, label, className }: TagsInputFieldProps) {
  return (
    <TagsInput value={values} onValueChange={onChangeAction} onValidate={(value) => value.length > 0} max={6} addOnPaste className={className}>
      <TagsInputLabel>{label}</TagsInputLabel>
      <TagsInputList>
        {values.map((tag) => (
          <TagsInputItem key={tag} value={tag}>
            {tag}
          </TagsInputItem>
        ))}
        <TagsInputInput
          placeholder="タグを追加..."
          onKeyDown={(e) => {
            // 日本語入力の Enter を無視
            if (e.key === "Enter" && e.nativeEvent.isComposing) {
              e.preventDefault();
              e.stopPropagation();
            }
          }}
        />
      </TagsInputList>
    </TagsInput>
  );
}
