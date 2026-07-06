import { httpClient } from "@/lib/client/httpClient";
import { SolutionComment, SolutionLikeStatus, SolutionVotesCount } from "@/shared/model/solution";
import { Solution, SolutionResponse } from "@/shared/model/solutionCreate";

export const solutionApi = {
  create: async (solution: Solution, token: string): Promise<SolutionResponse> => {
    if (!token) {
      throw new Error("投稿にはログインが必要です。");
    }

    const resp = await httpClient.postWithToken<SolutionResponse>("api/solutions", token, JSON.stringify(solution));
    if (resp.ok) {
      return resp.data;
    }
    const message = typeof resp.error === "string" && resp.error.length > 0 ? resp.error : "投稿に失敗しました。";
    throw new Error(`${message} (status: ${resp.status})`);
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
    if (resp.ok) {
      return resp.data.liked;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return false;
  },

  vote: async (solutionId: string, token: string): Promise<boolean> => {
    const resp = await httpClient.postWithToken<SolutionLikeStatus>(
      "api/solutions/votes",
      token,
      JSON.stringify({ solutionId }),
    );
    if (resp.ok) {
      return resp.data.liked;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return false;
  },

  unvote: async (solutionId: string, token: string): Promise<boolean> => {
    const resp = await httpClient.deleteWithToken<SolutionLikeStatus>("api/solutions/votes", token, { solutionId });
    if (resp.ok) {
      return resp.data.liked;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return true;
  },

  getCommentsBySolutionId: async (solutionId: string): Promise<SolutionComment[]> => {
    const resp = await httpClient.get<SolutionComment[]>("api/solutions/comments", { solutionId });
    if (resp.ok) {
      return resp.data;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return [];
  },

  createComment: async (solutionId: string, bodyMd: string, token: string): Promise<SolutionComment | null> => {
    const resp = await httpClient.postWithToken<SolutionComment>(
      "api/solutions/comments",
      token,
      JSON.stringify({ solutionId, bodyMd }),
    );
    if (resp.ok) {
      return resp.data;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return null;
  },

  updateComment: async (commentId: string, bodyMd: string, token: string): Promise<SolutionComment | null> => {
    const resp = await httpClient.patchWithToken<SolutionComment>(
      "api/solutions/comments",
      token,
      JSON.stringify({ commentId, bodyMd }),
    );
    if (resp.ok) {
      return resp.data;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return null;
  },

  deleteComment: async (commentId: string, token: string): Promise<boolean> => {
    const resp = await httpClient.deleteWithToken<{ commentId: string }>("api/solutions/comments", token, { commentId });
    if (resp.ok) {
      return true;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return false;
  },

  update: async (
    solutionId: string,
    title: string,
    bodyMd: string,
    submitUrl: string,
    tags: string[],
    token: string,
  ): Promise<boolean> => {
    const resp = await httpClient.patchWithToken<SolutionResponse>(
      "api/solutions",
      token,
      JSON.stringify({ solutionId, title, bodyMd, submitUrl, tags }),
    );
    if (resp.ok) {
      return true;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return false;
  },

  delete: async (solutionId: string, token: string): Promise<boolean> => {
    const resp = await httpClient.deleteWithToken<{ solutionId: string }>("api/solutions", token, { solutionId });
    if (resp.ok) {
      return true;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return false;
  },
};
