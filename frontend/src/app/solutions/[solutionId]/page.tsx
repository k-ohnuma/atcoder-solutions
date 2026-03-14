import { notFound } from "next/navigation";
import { SolutionDetail } from "@/features/solutions/ui/organisms/SolutionDetail";
import { ApiClient } from "@/lib/server/apiClient";
import { serverConfig } from "@/shared/config/backend";

const apiClient = new ApiClient(serverConfig.appConfig.apiBaseEndpoint);

type PageProps = {
  params: Promise<{
    solutionId: string;
  }>;
};

export default async function SolutionPage({ params }: PageProps) {
  const { solutionId } = await params;
  const solutionResp = await apiClient.getSolutionById(solutionId);
  const votesCount = await apiClient.getSolutionVotesCount(solutionId);
  const comments = await apiClient.getCommentsBySolutionId(solutionId);

  if (!solutionResp.ok) {
    if (solutionResp.status === 404) {
      notFound();
    }
    throw new Error(`failed to fetch solution: status=${solutionResp.status}, error=${solutionResp.error}`);
  }

  const solution = solutionResp.data;

  return (
    <SolutionDetail
      solutionId={solution.id}
      title={solution.title}
      problemId={solution.problemId}
      problemTitle={solution.problemTitle}
      userId={solution.userId}
      userName={solution.userName}
      tags={solution.tags}
      bodyMd={solution.bodyMd}
      submitUrl={solution.submitUrl}
      createdAt={solution.createdAt}
      initialVotesCount={votesCount}
      initialComments={comments}
    />
  );
}
