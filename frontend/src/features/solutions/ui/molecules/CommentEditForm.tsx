import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { MarkdownRenderer } from "../atoms/MarkdownRenderer";
import { MarkdownEditor } from "./MarkdownEditor";

type CommentEditFormProps = {
  bodyMd: string;
  errorMessage?: string;
  isSubmitting: boolean;
  onBodyChangeAction: (value: string) => void;
  onSubmitAction: () => void;
  onCancelAction: () => void;
};

export function CommentEditForm({
  bodyMd,
  errorMessage,
  isSubmitting,
  onBodyChangeAction,
  onSubmitAction,
  onCancelAction,
}: CommentEditFormProps) {
  return (
    <div className="space-y-2">
      <Tabs defaultValue="write" className="gap-2">
        <TabsList>
          <TabsTrigger value="write">入力</TabsTrigger>
          <TabsTrigger value="preview">プレビュー</TabsTrigger>
        </TabsList>
        <TabsContent value="write">
          <div className="h-40">
            <MarkdownEditor value={bodyMd} onChangeAction={onBodyChangeAction} />
          </div>
        </TabsContent>
        <TabsContent value="preview">
          <div className="min-h-40 rounded-md border bg-muted/30 p-2">
            <MarkdownRenderer value={bodyMd || "（プレビューはここに表示されます）"} />
          </div>
        </TabsContent>
      </Tabs>
      {errorMessage && <p className="text-xs text-destructive">{errorMessage}</p>}
      <div className="flex items-center gap-2">
        <Button type="button" size="sm" disabled={isSubmitting} onClick={onSubmitAction}>
          {isSubmitting ? "更新中..." : "更新"}
        </Button>
        <Button type="button" size="sm" variant="outline" disabled={isSubmitting} onClick={onCancelAction}>
          キャンセル
        </Button>
      </div>
    </div>
  );
}
