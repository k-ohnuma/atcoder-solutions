"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useRouter } from "next/navigation";
import React, { useMemo, useState } from "react";
import { useForm } from "react-hook-form";
import { Button } from "@/components/ui/button";
import { useContestsBySeries } from "@/lib/client/contest/useContestsBySeries";
import { useProblemsByContest } from "@/lib/client/problem/useProblemsByContest";
import { useCreateSolution } from "@/lib/client/solution/useCreateSolution";
import { CreateSolutionInput, createSolutionSchema } from "@/shared/model/solutionCreate";
import { TextInput } from "../atoms/TextInput";
import { ContestProblemSelectorSection, MarkdownEditor, MarkdownPreviewPanel, TagsInputField } from "../molecules";

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

  const createMutation = useCreateSolution();
  const router = useRouter();

  return (
    <form
      onSubmit={handleSubmit(async (v) => {
        const id = await createMutation.mutateAsync(v);
        const solutionId = id.solutionId;
        if (!solutionId) {
          return;
        }
        router.push(`/solutions/${solutionId}`);
      })}
    >
      <div className="flex h-[calc(100vh-80px)] flex-col gap-4 p-4">
        <ContestProblemSelectorSection
          series={series}
          onSeriesChangeAction={(value) => {
            setSeries(value);
            setContestId(null);
            setValue("problemId", "", { shouldValidate: true });
          }}
          contests={contests}
          contestsLoading={contestsLoading}
          contestsIsError={contestsIsError}
          contestId={contestId}
          onContestChangeAction={(value) => {
            setContestId(value);
            setValue("problemId", "", { shouldValidate: true });
          }}
          problems={problems}
          problemsLoading={problemsLoading}
          problemsIsError={problemsIsError}
          selectedProblemId={problemId}
          selectedProblem={selectedProblem}
          onProblemChangeAction={(id) => setValue("problemId", id, { shouldValidate: true })}
          problemErrorMessage={errors.problemId?.message}
        />
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

          <MarkdownPreviewPanel
            title={title}
            bodyMd={bodyMd}
            contestId={contestId}
            selectedProblemIndex={selectedProblem?.problemIndex}
            selectedProblemTitle={selectedProblem?.title}
          />
        </div>
        <div className="flex items-center justify-end gap-2">
          <Button type="submit" disabled={isSubmitting || createMutation.isPending}>
            {isSubmitting || createMutation.isPending ? "送信中..." : "投稿する"}
          </Button>
        </div>
      </div>
    </form>
  );
};
