"use client";

import React, { useEffect, useMemo, useState } from "react";
import { MarkdownEditor } from "../molecules";
import { MarkdownRenderer } from "../atoms";
import { TagsInputField } from "../molecules/TagInput";
import { TextInput } from "../atoms/TextInput";

import { ContestCombobox } from "../molecules/ContestCombobox"; // 前に貼ったやつ
import { Button } from "@/components/ui/button";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { cn } from "@/lib/utils";
import { useContestsBySeries } from "@/lib/client/contest/useContestsBySeries";
import { useProblemsByContest } from "@/lib/client/problem/useProblemsByContest";
import { Problem } from "@/shared/model/problem";

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
  const [tags, setTags] = useState<string[]>([]);
  const [text, setText] = useState("");
  const [title, setTitle] = useState("");
  const [submitUrl, setSubmitUrl] = useState("");
  const [series, setSeries] = useState<string>("ABC");
  const [contestId, setContestId] = useState<string | null>(null);
  const [problemId, setProblemId] = useState<string | null>(null);

  const {
    data: contests = [],
    isLoading: contestsLoading,
    isError: contestsIsError,
    // error: contestsError,
  } = useContestsBySeries(series);

  const {
    data: problems = [],
    isLoading: problemsLoading,
    isError: problemsIsError,
    // error: problemsError,
  } = useProblemsByContest(contestId);

  useEffect(() => {
    setProblemId(null);
  }, [contestId]);

  const selectedProblem = useMemo(() => problems.find((p) => p.id === problemId) ?? null, [problems, problemId]);

  return (
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
                setProblemId(null);
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
              onChangeAction={setContestId}
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
            onChangeAction={setProblemId}
            disabled={!contestId || problemsLoading}
            isLoading={problemsLoading}
          />

          {problemsIsError && <div className="text-sm text-destructive">問題取得に失敗</div>}
        </div>
      </div>

      <TextInput value={title} onChangeAction={setTitle} placeholder="タイトル" className="w-full" />
      <TagsInputField values={tags} onChangeAction={setTags} label="タグ" className="w-full" />
      <TextInput value={submitUrl} onChangeAction={setSubmitUrl} placeholder="提出URL" className="w-full" />

      <div className="grid flex-1 grid-cols-1 gap-4 md:grid-cols-2 min-h-[1000px]">
        <MarkdownEditor value={text} onChangeAction={setText} />
        <div className="rounded-xl border border-border bg-muted/40 p-4">
          <div className="mb-3 space-y-1">
            <div className="text-xs font-medium text-muted-foreground">Preview</div>
            <div className="text-sm font-semibold">{title || "（タイトル未設定）"}</div>
            {selectedProblem && (
              <div className="text-xs text-muted-foreground">
                {contestId?.toUpperCase()} / {selectedProblem.problemIndex}: {selectedProblem.title}
              </div>
            )}
          </div>
          <MarkdownRenderer value={text} />
        </div>
      </div>
    </div>
  );
};
