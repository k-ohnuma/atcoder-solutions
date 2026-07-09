import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { SupportedSeries } from "@/features/problems/model/contestProblemGroup";
import { ButtonLink } from "@/shared/ui/ButtonLink";

type ProblemSearchFormProps = {
  selectedSeries: SupportedSeries;
  query: string;
};

export function ProblemSearchForm({ selectedSeries, query }: ProblemSearchFormProps) {
  return (
    <form method="GET" className="flex w-full items-center gap-2">
      <input type="hidden" name="series" value={selectedSeries} />
      <Input name="q" placeholder="検索" defaultValue={query} />
      <Button type="submit" variant="outline">
        検索
      </Button>
      {query && (
        <ButtonLink href={`/?series=${selectedSeries}`} variant="ghost">
          クリア
        </ButtonLink>
      )}
    </form>
  );
}
