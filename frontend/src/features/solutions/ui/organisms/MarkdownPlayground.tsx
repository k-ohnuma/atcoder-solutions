"use client";

import React, { useState } from "react";
import { MarkdownEditor } from "../molecules";
import { MarkdownRenderer } from "../atoms";
import { TagsInputField } from "../molecules/TagInput";
import { TextInput } from "../atoms/TextInput";
import { Label } from "@radix-ui/react-label";

export const MarkdownPlayground: React.FC = () => {
  const [tags, setTags] = useState<string[]>([]);
  const [text, setText] = useState("");
  const [title, setTitle] = useState("");
  const [submitUrl, setSubmitUrl] = useState("");

  return (
    <div className="flex h-[calc(100vh-80px)] flex-col gap-4 p-4">
      <TextInput value={title} onChangeAction={setTitle} placeholder="タイトル" className="w-full" />
      <TagsInputField values={tags} onChangeAction={setTags} label="タグ" className="w-full" />
      <TextInput value={submitUrl} onChangeAction={setSubmitUrl} placeholder="提出URL" className="w-full" />

      <div className="grid flex-1 grid-cols-1 gap-4 md:grid-cols-2">
        <MarkdownEditor value={text} onChangeAction={setText} />
        <MarkdownRenderer value={text} />
      </div>
    </div>
  );
};
