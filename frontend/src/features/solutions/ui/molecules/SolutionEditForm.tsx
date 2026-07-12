import { Controller, UseFormReturn } from "react-hook-form";
import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { UpdateSolutionFormInput } from "@/features/solutions/model/updateSolution";
import { MarkdownRenderer } from "../atoms/MarkdownRenderer";
import { MarkdownEditor } from "./MarkdownEditor";
import { TagsInputField } from "./TagInput";

type SolutionEditFormProps = {
  form: UseFormReturn<UpdateSolutionFormInput>;
  isSubmitting: boolean;
  onSubmitAction: (values: UpdateSolutionFormInput) => void;
};

export function SolutionEditForm({ form, isSubmitting, onSubmitAction }: SolutionEditFormProps) {
  const {
    register,
    handleSubmit,
    control,
    formState: { errors },
  } = form;

  return (
    <form className="mt-3 space-y-2" onSubmit={handleSubmit(onSubmitAction)}>
      <div className="grid items-start gap-2 md:grid-cols-[120px_1fr]">
        <p className="pt-2 text-sm font-medium text-muted-foreground">タイトル</p>
        <div>
          <input className="w-full rounded-md border px-3 py-2 text-sm" {...register("title")} placeholder="タイトル" />
          {errors.title && <p className="mt-1 text-xs text-destructive">{errors.title.message}</p>}
        </div>
      </div>
      <div className="grid items-start gap-2 md:grid-cols-[120px_1fr]">
        <p className="pt-2 text-sm font-medium text-muted-foreground">提出URL</p>
        <div>
          <input
            className="w-full rounded-md border px-3 py-2 text-sm"
            {...register("submitUrl")}
            placeholder="提出URL（任意）"
          />
          {errors.submitUrl && <p className="mt-1 text-xs text-destructive">{errors.submitUrl.message}</p>}
        </div>
      </div>
      <div className="grid items-start gap-2 md:grid-cols-[120px_1fr]">
        <p className="pt-2 text-sm font-medium text-muted-foreground">タグ</p>
        <div>
          <Controller
            control={control}
            name="tags"
            render={({ field }) => (
              <TagsInputField label="タグ" values={field.value} onChangeAction={field.onChange} className="w-full" hideLabel />
            )}
          />
          {errors.tags && <p className="mt-1 text-xs text-destructive">{errors.tags.message}</p>}
        </div>
      </div>
      <div className="grid items-start gap-2 md:grid-cols-[120px_1fr]">
        <p className="pt-2 text-sm font-medium text-muted-foreground">本文</p>
        <div>
          <Controller
            control={control}
            name="bodyMd"
            render={({ field }) => (
              <Tabs defaultValue="write" className="gap-2">
                <TabsList>
                  <TabsTrigger value="write">入力</TabsTrigger>
                  <TabsTrigger value="preview">プレビュー</TabsTrigger>
                </TabsList>
                <TabsContent value="write">
                  <div className="h-56">
                    <MarkdownEditor value={field.value} onChangeAction={field.onChange} />
                  </div>
                </TabsContent>
                <TabsContent value="preview">
                  <div className="min-h-56 rounded-md border bg-muted/30 p-3">
                    <MarkdownRenderer value={field.value || "（プレビューはここに表示されます）"} />
                  </div>
                </TabsContent>
              </Tabs>
            )}
          />
          {errors.bodyMd && <p className="mt-1 text-xs text-destructive">{errors.bodyMd.message}</p>}
        </div>
      </div>
      <div className="flex items-center gap-2">
        <Button type="submit" size="sm" disabled={isSubmitting}>
          {isSubmitting ? "保存中..." : "更新を保存"}
        </Button>
      </div>
    </form>
  );
}
