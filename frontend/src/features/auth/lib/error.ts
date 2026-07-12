export function getAuthFormErrorMessage(error: unknown) {
  return error instanceof Error ? error.message : "認証処理でエラーが発生しました。時間をおいて再度お試しください。";
}
