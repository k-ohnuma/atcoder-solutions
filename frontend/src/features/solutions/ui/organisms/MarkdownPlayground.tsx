"use client";

import React, { useMemo, useState } from "react";
import { Button } from "@/components/ui/button";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useContestsBySeries } from "@/lib/client/contest/useContestsBySeries";
import { useProblemsByContest } from "@/lib/client/problem/useProblemsByContest";
import { cn } from "@/lib/utils";
import { Problem } from "@/shared/model/problem";
import { MarkdownRenderer } from "../atoms";
import { TextInput } from "../atoms/TextInput";
import { MarkdownEditor } from "../molecules";
import { ContestCombobox } from "../molecules/ContestCombobox"; // 前に貼ったやつ
import { TagsInputField } from "../molecules/TagInput";
import { CreateSolutionInput, createSolutionSchema } from "@/shared/model/solutionCreate";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";

function ProblemButtonSelect({
  items,
  value,
  onChangeAction,
  disabled,
  isLoading,
}: {
  items: Problem[];
  value: string | null;
  onChangeAction: (id: string) => void;
  disabled?: boolean;
  isLoading?: boolean;
}) {
  if (isLoading) return <div className="text-sm text-muted-foreground">問題を読み込み中...</div>;

  return (
    <div className={cn("grid grid-cols-2 gap-2 sm:grid-cols-3", disabled && "opacity-60")}>
      {items.map((p) => {
        const selected = p.id === value;
        return (
          <Button
            key={p.id}
            type="button"
            variant={selected ? "default" : "outline"}
            className="justify-start"
            disabled={disabled}
            onClick={() => onChangeAction(p.id)}
          >
            <span className="mr-2 text-xs">{p.problemIndex}</span>
            <span className="truncate">{p.title}</span>
          </Button>
        );
      })}
    </div>
  );
}

export const MarkdownPlayground: React.FC = () => {
  const [series, setSeries] = useState<string>("ABC");
  const [contestId, setContestId] = useState<string | null>(null);
  const form = useForm<CreateSolutionInput>({
    resolver: zodResolver(createSolutionSchema),
    defaultValues: {
      problemId: "",
      title: "",
      tags: [],
      submitUrl: "",
      bodyMd: "",
    },
    mode: "onChange",
  });

  const {
    watch,
    setValue,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = form;

  const problemId = watch("problemId") || null;
  const title = watch("title");
  const bodyMd = watch("bodyMd");
  const tags = watch("tags");
  const submitUrl = watch("submitUrl");

  const { data: contests = [], isLoading: contestsLoading, isError: contestsIsError } = useContestsBySeries(series);

  const { data: problems = [], isLoading: problemsLoading, isError: problemsIsError } = useProblemsByContest(contestId);

  const selectedProblem = useMemo(() => problems.find((p) => p.id === problemId) ?? null, [problems, problemId]);

  return (
    <form onSubmit={handleSubmit((v) => console.log(v))}>
      <div className="flex h-[calc(100vh-80px)] flex-col gap-4 p-4">
        <div className="rounded-2xl border border-border bg-card p-4 shadow-sm">
          <div className="grid gap-3 md:grid-cols-2">
            <div className="space-y-2">
              <div className="text-xs font-medium text-muted-foreground">コンテスト種別</div>
              <Tabs
                value={series}
                onValueChange={(v) => {
                  setSeries(v);
                  setContestId(null);
                  setValue("problemId", "", { shouldValidate: true });
                }}
              >
                <TabsList className="grid w-full grid-cols-4">
                  <TabsTrigger value="ABC">ABC</TabsTrigger>
                  <TabsTrigger value="ARC">ARC</TabsTrigger>
                  <TabsTrigger value="AGC">AGC</TabsTrigger>
                  <TabsTrigger value="OTHER">OTHER</TabsTrigger>
                </TabsList>
              </Tabs>
            </div>

            <div className="space-y-2">
              <div className="text-xs font-medium text-muted-foreground">コンテスト名</div>
              <ContestCombobox
                items={contests}
                value={contestId}
                onChangeAction={(v) => {
                  setContestId(v);
                  setValue("problemId", "", { shouldValidate: true });
                }}
                isLoading={contestsLoading}
                disabled={contestsLoading}
                placeholder="コンテストを選択"
              />
              {contestsIsError && <div className="text-sm text-destructive">コンテスト取得に失敗</div>}
            </div>
          </div>

          <div className="mt-4 space-y-2">
            <div className="flex items-center justify-between">
              <div className="text-xs font-medium text-muted-foreground">問題</div>
              {selectedProblem && (
                <div className="text-xs text-muted-foreground">
                  選択中: <span className="font-medium text-foreground">{selectedProblem.id}</span>
                </div>
              )}
            </div>

            <ProblemButtonSelect
              items={problems}
              value={problemId}
              onChangeAction={(id) => setValue("problemId", id, { shouldValidate: true })}
              disabled={!contestId || problemsLoading}
              isLoading={problemsLoading}
            />
            {errors.problemId && <p className="text-xs text-destructive">{errors.problemId.message}</p>}
            {problemsIsError && <div className="text-sm text-destructive">問題取得に失敗</div>}
          </div>
        </div>
        <div className="space-y-1">
          <TextInput
            value={title}
            onChangeAction={(v) => setValue("title", v, { shouldValidate: true })}
            placeholder="タイトル"
            className="w-full"
          />
          {errors.title && <p className="text-xs text-destructive">{errors.title.message}</p>}
        </div>
        <div className="space-y-1">
          <TagsInputField
            values={tags}
            onChangeAction={(v) => setValue("tags", v, { shouldValidate: true })}
            label="タグ"
            className="w-full"
          />
          {errors.tags && <p className="text-xs text-destructive">{errors.tags.message}</p>}
        </div>
        <div className="space-y-1">
          <TextInput
            value={submitUrl}
            onChangeAction={(v) => setValue("submitUrl", v, { shouldValidate: true })}
            placeholder="提出URL"
            className="w-full"
          />
          {errors.submitUrl && <p className="text-xs text-destructive">{errors.submitUrl.message}</p>}
        </div>

        <div className="grid flex-1 min-h-[800px] grid-cols-1 gap-4 md:grid-cols-2">
          <div className="flex h-full min-h-[800px] flex-col rounded-xl border border-border">
            <MarkdownEditor value={bodyMd} onChangeAction={(v) => setValue("bodyMd", v, { shouldValidate: true })} />
            {errors.bodyMd && <p className="px-3 pb-3 text-xs text-destructive">{errors.bodyMd.message}</p>}
          </div>

          <div className="flex h-full min-h-[800px] flex-col rounded-xl border border-border bg-muted/40">
            <div className="border-b px-3 py-2">
              <div className="text-sm font-semibold">{title || "（タイトル未設定）"}</div>
              {selectedProblem && (
                <div className="text-xs text-muted-foreground">
                  {contestId?.toUpperCase()} / {selectedProblem.problemIndex}: {selectedProblem.title}
                </div>
              )}
            </div>
            <div className="flex-1 overflow-y-auto p-4">
              <MarkdownRenderer value={bodyMd} />
            </div>
          </div>
        </div>
        <div className="flex items-center justify-end gap-2">
          <Button type="submit" disabled={isSubmitting}>
            投稿する
          </Button>
        </div>
      </div>
    </form>
  );
};
