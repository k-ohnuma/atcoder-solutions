export function buildAtcoderProblemUrl(problemId: string, contestCode?: string): string {
  const contestId = contestCode ?? problemId.split("_")[0];
  return `https://atcoder.jp/contests/${contestId}/tasks/${problemId}`;
}
