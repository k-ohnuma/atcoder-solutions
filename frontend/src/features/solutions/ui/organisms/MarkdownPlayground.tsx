"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useRouter, useSearchParams } from "next/navigation";
import React, { useEffect, useMemo, useState } from "react";
import { useForm } from "react-hook-form";
import { PageContainer } from "@/components/layout/PageContainer";
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
  const [submitError, setSubmitError] = useState<string | null>(null);
  const [prefilled, setPrefilled] = useState(false);
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
  const searchParams = useSearchParams();

  useEffect(() => {
    if (prefilled) {
      return;
    }
    const initialProblemId = searchParams.get("problemId");
    if (!initialProblemId) {
      setPrefilled(true);
      return;
    }

    const normalizedProblemId = initialProblemId.trim().toLowerCase();
    const contest = normalizedProblemId.split("_")[0];
    if (!contest) {
      setPrefilled(true);
      return;
    }

    const upperContest = contest.toUpperCase();
    const nextSeries = upperContest.startsWith("ABC")
      ? "ABC"
      : upperContest.startsWith("ARC")
        ? "ARC"
        : upperContest.startsWith("AGC")
          ? "AGC"
          : "OTHER";

    setSeries(nextSeries);
    setContestId(contest);
    setValue("problemId", normalizedProblemId, { shouldValidate: true });
    setPrefilled(true);
  }, [prefilled, searchParams, setValue]);

  return (
    <form
      onSubmit={handleSubmit(async (v) => {
        setSubmitError(null);
        try {
          const id = await createMutation.mutateAsync(v);
          const solutionId = id.solutionId;
          if (!solutionId) {
            setSubmitError("投稿に失敗しました。時間をおいて再度お試しください。");
            return;
          }
          router.push(`/solutions/${solutionId}`);
        } catch (e) {
          const message = e instanceof Error ? e.message : "投稿に失敗しました。時間をおいて再度お試しください。";
          setSubmitError(message);
        }
      })}
    >
      <PageContainer as="div" className="flex min-h-[calc(100vh-80px)] flex-col gap-4 md:py-6">
        <div className="sticky top-16 z-20 rounded-xl border border-border bg-background/95 p-3 shadow-sm backdrop-blur">
          <div className="flex items-center justify-between gap-3">
            <div className="min-w-0">
              <p className="truncate text-sm font-semibold">{title || "新規解説"}</p>
              <p className="text-xs text-muted-foreground">
                {selectedProblem ? `${selectedProblem.id} の解説` : "問題と内容を入力して投稿"}
              </p>
            </div>
            <Button type="submit" disabled={isSubmitting || createMutation.isPending}>
              {isSubmitting || createMutation.isPending ? "送信中..." : "投稿する"}
            </Button>
          </div>
          {submitError && <p className="mt-2 text-sm text-destructive">{submitError}</p>}
        </div>

        <section className="space-y-4 rounded-2xl border border-border bg-card p-4">
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
          <div className="grid gap-3 md:grid-cols-2">
            <div className="space-y-1 md:col-span-2">
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
                placeholder="提出URL（任意）"
                className="w-full"
              />
              {errors.submitUrl && <p className="text-xs text-destructive">{errors.submitUrl.message}</p>}
            </div>
          </div>
        </section>

        <div className="grid min-h-[65vh] grid-cols-1 gap-4 md:grid-cols-2">
          <section className="flex min-h-[65vh] flex-col rounded-xl border border-border bg-background">
            <div className="border-b px-3 py-2 text-xs font-semibold text-muted-foreground">本文（Markdown）</div>
            <div className="flex-1 p-3">
              <MarkdownEditor value={bodyMd} onChangeAction={(v) => setValue("bodyMd", v, { shouldValidate: true })} />
            </div>
            {errors.bodyMd && <p className="px-3 pb-3 text-xs text-destructive">{errors.bodyMd.message}</p>}
          </section>

          <section className="min-h-[65vh]">
            <MarkdownPreviewPanel
              title={title}
              bodyMd={bodyMd}
              contestId={contestId}
              selectedProblemIndex={selectedProblem?.problemIndex}
              selectedProblemTitle={selectedProblem?.title}
            />
          </section>
        </div>
      </PageContainer>
    </form>
  );
};
