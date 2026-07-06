import { notFound } from "next/navigation";
import { SolutionDetail } from "@/features/solutions/ui/organisms/SolutionDetail";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";

const solutionRepository = new SolutionRepositoryImpl();

type PageProps = {
  params: Promise<{
    solutionId: string;
  }>;
};

export default async function SolutionPage({ params }: PageProps) {
  const { solutionId } = await params;
  const solutionResp = await solutionRepository.getBySolutionId(solutionId);
  const votesCountResp = await solutionRepository.getVotesCount(solutionId);
  const commentsResp = await solutionRepository.getCommentsBySolutionId(solutionId);

  if (!solutionResp.ok) {
    if (solutionResp.status === 404) {
      notFound();
    }
    throw new Error(`failed to fetch solution: status=${solutionResp.status}, error=${solutionResp.error}`);
  }

  const solution = solutionResp.data;
  const votesCount = votesCountResp.ok ? votesCountResp.data.votesCount : 0;
  const comments = commentsResp.ok ? commentsResp.data : [];

  return (
    <SolutionDetail
      solutionId={solution.id}
      title={solution.title}
      problemId={solution.problemId}
      contestCode={solution.contestCode}
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
