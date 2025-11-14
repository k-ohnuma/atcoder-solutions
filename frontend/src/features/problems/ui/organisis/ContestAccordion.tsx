import { Accordion } from "@/components/ui/accordion";
import { Problem } from "@/server/domain/problems";
import { ChevronDownIcon } from "lucide-react";
import { ProblemsList } from "./ProblemList";

export function ContestAccordion({ problemsMap }: { problemsMap: Map<string, Problem[]> }) {
  const list = [...problemsMap.entries()];
  return (
    <Accordion.Root multiple>
      {list.map(([contestCode, problems]) => (
        <Accordion.Item key={contestCode} value={contestCode}>
          <Accordion.ItemTrigger>
            {contestCode}
            <Accordion.ItemIndicator>
              <ChevronDownIcon />
            </Accordion.ItemIndicator>
          </Accordion.ItemTrigger>
          <Accordion.ItemContent>
            <ProblemsList problems={problems} />
          </Accordion.ItemContent>
        </Accordion.Item>
      ))}
    </Accordion.Root>
  );
}
