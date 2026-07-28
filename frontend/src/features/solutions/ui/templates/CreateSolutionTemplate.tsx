"use client";

import React from "react";
import { PageContainer } from "@/components/layout/PageContainer";
import { useCreateSolutionForm } from "@/features/solutions/hooks/useCreateSolutionForm";
import { clientConfig } from "@/shared/config/client";
import { TurnstileField } from "@/shared/turnstile/TurnstileField";
import { ContestProblemSelectorSection } from "../molecules/ContestProblemSelectorSection";
import { CreateSolutionBodyEditor } from "../molecules/CreateSolutionBodyEditor";
import { CreateSolutionHeader } from "../molecules/CreateSolutionHeader";
import { CreateSolutionMetadataFields } from "../molecules/CreateSolutionMetadataFields";

export const CreateSolutionTemplate: React.FC = () => {
  const {
    values,
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
    isSubmitting,
    turnstileToken,
    setTurnstileToken,
    turnstileResetKey,
    submit,
    selectSeries,
    selectContest,
    selectProblem,
    setTitle,
    setTags,
    setSubmitUrl,
    setBodyMd,
  } = useCreateSolutionForm();
  const { problemId, title, bodyMd, tags, submitUrl } = values;

  return (
    <form onSubmit={submit}>
      <PageContainer as="div" className="flex min-h-[calc(100vh-80px)] flex-col gap-4 md:py-6">
        <CreateSolutionHeader
          title={title}
          selectedProblem={selectedProblem}
          submitError={submitError}
          isSubmitting={isSubmitting}
          canSubmit={!clientConfig.turnstileConfig.enabled || !!turnstileToken}
        />

        <section className="space-y-4 p-1">
          <ContestProblemSelectorSection
            series={series}
            onSeriesChangeAction={selectSeries}
            contests={contests}
            contestsLoading={contestsLoading}
            contestsIsError={contestsIsError}
            contestId={contestId}
            onContestChangeAction={selectContest}
            problems={problems}
            problemsLoading={problemsLoading}
            problemsIsError={problemsIsError}
            selectedProblemId={problemId}
            onProblemChangeAction={selectProblem}
            problemErrorMessage={errors.problemId?.message}
          />
          <CreateSolutionMetadataFields
            title={title}
            tags={tags}
            submitUrl={submitUrl}
            errors={errors}
            onTitleChangeAction={setTitle}
            onTagsChangeAction={setTags}
            onSubmitUrlChangeAction={setSubmitUrl}
          />
        </section>

        <div className="px-1">
          <TurnstileField key={turnstileResetKey} onTokenChangeAction={setTurnstileToken} />
        </div>

        <CreateSolutionBodyEditor title={title} bodyMd={bodyMd} error={errors.bodyMd} onBodyChangeAction={setBodyMd} />
      </PageContainer>
    </form>
  );
};
