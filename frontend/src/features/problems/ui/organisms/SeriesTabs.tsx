import { SupportedSeries, supportedSeries } from "@/features/problems/model/contestProblemGroup";
import { ButtonLink } from "@/shared/ui/ButtonLink";

type SeriesTabsProps = {
  selectedSeries: SupportedSeries;
};

export function SeriesTabs({ selectedSeries }: SeriesTabsProps) {
  return (
    <div className="flex flex-wrap gap-2">
      {supportedSeries.map((code) => {
        const active = code === selectedSeries;
        return (
          <ButtonLink key={code} href={`/?series=${code}`} size="sm" variant={active ? "default" : "outline"}>
            {code}
          </ButtonLink>
        );
      })}
    </div>
  );
}
