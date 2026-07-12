import { SolutionListSortBy } from "@/shared/model/solution";
import { ButtonLink } from "@/shared/ui/ButtonLink";

type SolutionSortLinksProps = {
  selectedSort: SolutionListSortBy;
  latestHref: string;
  votesHref: string;
};

export function SolutionSortLinks({ selectedSort, latestHref, votesHref }: SolutionSortLinksProps) {
  return (
    <div className="mb-6 flex items-center gap-2 text-sm">
      <ButtonLink href={latestHref} size="sm" variant={selectedSort === "latest" ? "default" : "outline"}>
        新着順
      </ButtonLink>
      <ButtonLink href={votesHref} size="sm" variant={selectedSort === "votes" ? "default" : "outline"}>
        いいね順
      </ButtonLink>
    </div>
  );
}
