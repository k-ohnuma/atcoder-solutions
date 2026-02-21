export function buildAtcoderProblemUrl(problemId: string): string {
  const contestId = problemId.split("_")[0];
  return `https://atcoder.jp/contests/${contestId}/tasks/${problemId}`;
}
