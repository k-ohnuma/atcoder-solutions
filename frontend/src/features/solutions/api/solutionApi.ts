import { httpClient, Resp } from "@/lib/client/httpClient";
import { SolutionComment, SolutionLikeStatus, SolutionVotesCount } from "@/shared/model/solution";
import { Solution, SolutionResponse } from "@/shared/model/solutionCreate";

function getApiErrorMessage<T>(resp: Resp<T>, fallbackMessage: string) {
  if (resp.ok) {
    return null;
  }
  const message = typeof resp.error === "string" && resp.error.length > 0 ? resp.error : fallbackMessage;
  return `${message} (status: ${resp.status})`;
}

function throwIfFailed<T>(resp: Resp<T>, fallbackMessage: string): asserts resp is { ok: true; data: T; status: 200 } {
  const message = getApiErrorMessage(resp, fallbackMessage);
  if (message) {
    throw new Error(message);
  }
}

export const solutionApi = {
  create: async (solution: Solution, token: string): Promise<SolutionResponse> => {
    if (!token) {
      throw new Error("投稿にはログインが必要です。");
    }

    const resp = await httpClient.postWithToken<SolutionResponse>("api/solutions", token, JSON.stringify(solution));
    throwIfFailed(resp, "投稿に失敗しました。");
    return resp.data;
  },

  getVotesCount: async (solutionId: string): Promise<number> => {
    const resp = await httpClient.get<SolutionVotesCount>("api/solutions/votes", { solutionId });
    if (resp.ok) {
      return resp.data.votesCount;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return 0;
  },

  getMyLikeStatus: async (solutionId: string, token: string): Promise<boolean> => {
    const resp = await httpClient.getWithToken<SolutionLikeStatus>("api/solutions/votes/me", token, { solutionId });
    throwIfFailed(resp, "いいね状態の取得に失敗しました。");
    return resp.data.liked;
  },

  vote: async (solutionId: string, token: string): Promise<boolean> => {
    const resp = await httpClient.postWithToken<SolutionLikeStatus>(
      "api/solutions/votes",
      token,
      JSON.stringify({ solutionId }),
    );
    throwIfFailed(resp, "いいねに失敗しました。");
    return resp.data.liked;
  },

  unvote: async (solutionId: string, token: string): Promise<boolean> => {
    const resp = await httpClient.deleteWithToken<SolutionLikeStatus>("api/solutions/votes", token, { solutionId });
    throwIfFailed(resp, "いいねの解除に失敗しました。");
    return resp.data.liked;
  },

  getCommentsBySolutionId: async (solutionId: string): Promise<SolutionComment[]> => {
    const resp = await httpClient.get<SolutionComment[]>("api/solutions/comments", { solutionId });
    if (resp.ok) {
      return resp.data;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return [];
  },

  createComment: async (solutionId: string, bodyMd: string, token: string): Promise<SolutionComment> => {
    const resp = await httpClient.postWithToken<SolutionComment>(
      "api/solutions/comments",
      token,
      JSON.stringify({ solutionId, bodyMd }),
    );
    throwIfFailed(resp, "コメントの投稿に失敗しました。");
    return resp.data;
  },

  updateComment: async (commentId: string, bodyMd: string, token: string): Promise<SolutionComment> => {
    const resp = await httpClient.patchWithToken<SolutionComment>(
      "api/solutions/comments",
      token,
      JSON.stringify({ commentId, bodyMd }),
    );
    throwIfFailed(resp, "コメントの更新に失敗しました。");
    return resp.data;
  },

  deleteComment: async (commentId: string, token: string): Promise<void> => {
    const resp = await httpClient.deleteWithToken<{ commentId: string }>("api/solutions/comments", token, { commentId });
    throwIfFailed(resp, "コメントの削除に失敗しました。");
  },

  update: async (
    solutionId: string,
    title: string,
    bodyMd: string,
    submitUrl: string,
    tags: string[],
    token: string,
  ): Promise<void> => {
    const resp = await httpClient.patchWithToken<SolutionResponse>(
      "api/solutions",
      token,
      JSON.stringify({ solutionId, title, bodyMd, submitUrl, tags }),
    );
    throwIfFailed(resp, "解説の更新に失敗しました。");
  },

  delete: async (solutionId: string, token: string): Promise<void> => {
    const resp = await httpClient.deleteWithToken<{ solutionId: string }>("api/solutions", token, { solutionId });
    throwIfFailed(resp, "解説の削除に失敗しました。");
  },
};
