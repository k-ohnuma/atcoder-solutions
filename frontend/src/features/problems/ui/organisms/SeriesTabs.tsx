import Link from "next/link";
import { Button } from "@/components/ui/button";
import { SupportedSeries, supportedSeries } from "@/features/problems/model/contestProblemGroup";

type SeriesTabsProps = {
  selectedSeries: SupportedSeries;
};

export function SeriesTabs({ selectedSeries }: SeriesTabsProps) {
  return (
    <div className="flex flex-wrap gap-2">
      {supportedSeries.map((code) => {
        const active = code === selectedSeries;
        return (
          <Button key={code} asChild size="sm" variant={active ? "default" : "outline"}>
            <Link href={`/?series=${code}`}>{code}</Link>
          </Button>
        );
      })}
    </div>
  );
}
