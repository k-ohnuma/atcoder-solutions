"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useRouter, useSearchParams } from "next/navigation";
import { useEffect, useMemo, useState } from "react";
import { useForm } from "react-hook-form";
import { useToast } from "@/components/ui/toast";
import { useContestsBySeries } from "@/features/contests/hooks/useContestsBySeries";
import { useProblemsByContest } from "@/features/problems/hooks/useProblemsByContest";
import { useCreateSolution } from "@/features/solutions/hooks/useCreateSolution";
import { CreateSolutionInput, createSolutionSchema } from "@/shared/model/solutionCreate";

function inferSeriesFromProblemId(problemId: string) {
  const contest = problemId.split("_")[0];
  const upperContest = contest.toUpperCase();

  if (upperContest.startsWith("ABC")) {
    return "ABC";
  }
  if (upperContest.startsWith("ARC")) {
    return "ARC";
  }
  if (upperContest.startsWith("AGC")) {
    return "AGC";
  }
  if (upperContest.startsWith("AHC")) {
    return "AHC";
  }
  if (upperContest.startsWith("AWC")) {
    return "AWC";
  }
  return "OTHER";
}

export function useCreateSolutionForm() {
  const { toast } = useToast();
  const router = useRouter();
  const searchParams = useSearchParams();
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

    setSeries(inferSeriesFromProblemId(normalizedProblemId));
    setContestId(contest);
    setValue("problemId", normalizedProblemId, { shouldValidate: true });
    setPrefilled(true);
  }, [prefilled, searchParams, setValue]);

  const selectSeries = (value: string) => {
    setSeries(value);
    setContestId(null);
    setValue("problemId", "", { shouldValidate: true });
  };

  const selectContest = (value: string | null) => {
    setContestId(value);
    setValue("problemId", "", { shouldValidate: true });
  };

  const submit = handleSubmit(async (values) => {
    setSubmitError(null);
    try {
      const response = await createMutation.mutateAsync(values);
      const solutionId = response.solutionId;
      if (!solutionId) {
        setSubmitError("投稿に失敗しました。時間をおいて再度お試しください。");
        toast({ title: "投稿に失敗しました", variant: "error" });
        return;
      }

      toast({ title: "解説を投稿しました" });
      router.push(`/solutions/${solutionId}`);
    } catch (error) {
      const message = error instanceof Error ? error.message : "投稿に失敗しました。時間をおいて再度お試しください。";
      setSubmitError(message);
      toast({ title: "投稿に失敗しました", description: message, variant: "error" });
    }
  });

  return {
    form,
    values: {
      problemId,
      title,
      bodyMd,
      tags,
      submitUrl,
    },
    errors,
    series,
    contestId,
    contests,
    contestsLoading,
    contestsIsError,
    problems,
    problemsLoading,
    problemsIsError,
    selectedProblem,
    submitError,
    isSubmitting: isSubmitting || createMutation.isPending,
    submit,
    selectSeries,
    selectContest,
    selectProblem: (id: string) => setValue("problemId", id, { shouldValidate: true }),
    setTitle: (value: string) => setValue("title", value, { shouldValidate: true }),
    setTags: (value: string[]) => setValue("tags", value, { shouldValidate: true }),
    setSubmitUrl: (value: string) => setValue("submitUrl", value, { shouldValidate: true }),
    setBodyMd: (value: string) => setValue("bodyMd", value, { shouldValidate: true }),
  };
}
