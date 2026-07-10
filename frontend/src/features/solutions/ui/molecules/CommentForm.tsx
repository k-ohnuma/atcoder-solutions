import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { MarkdownRenderer } from "../atoms/MarkdownRenderer";
import { MarkdownEditor } from "./MarkdownEditor";

type CommentFormProps = {
  bodyMd: string;
  errorMessage: string | null;
  isSubmitting: boolean;
  onBodyChangeAction: (value: string) => void;
  onSubmitAction: () => void;
};

export function CommentForm({ bodyMd, errorMessage, isSubmitting, onBodyChangeAction, onSubmitAction }: CommentFormProps) {
  return (
    <div className="rounded-xl bg-muted/30 p-4">
      <Tabs defaultValue="write" className="gap-3">
        <TabsList>
          <TabsTrigger value="write">入力</TabsTrigger>
          <TabsTrigger value="preview">プレビュー</TabsTrigger>
        </TabsList>
        <TabsContent value="write">
          <div className="h-48">
            <MarkdownEditor value={bodyMd} onChangeAction={onBodyChangeAction} />
          </div>
        </TabsContent>
        <TabsContent value="preview">
          <div className="min-h-48 rounded-lg border bg-muted/30 p-2">
            <MarkdownRenderer value={bodyMd || "（プレビューはここに表示されます）"} />
          </div>
        </TabsContent>
      </Tabs>

      <div className="mt-3 flex items-center justify-between gap-3">
        <p className="text-xs text-muted-foreground">{bodyMd.length}/2000</p>
        <Button type="button" size="sm" disabled={isSubmitting} onClick={onSubmitAction}>
          コメントを投稿
        </Button>
      </div>
      {errorMessage && <p className="mt-2 text-sm text-destructive">{errorMessage}</p>}
    </div>
  );
}
