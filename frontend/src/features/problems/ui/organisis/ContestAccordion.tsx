import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from "@/components/ui/accordion";
import { Problem } from "@/server/domain/problems";
import { ProblemsList } from "./ProblemList";

export function ContestAccordion({ problemsMap }: { problemsMap: Map<string, Problem[]> }) {
  const list = [...problemsMap.entries()];

  return (
    <Accordion type="multiple" className="w-full">
      {list.map(([contestCode, problems]) => (
        <AccordionItem key={contestCode} value={contestCode}>
          <AccordionTrigger>{contestCode}</AccordionTrigger>
          <AccordionContent>
            <ProblemsList problems={problems} />
          </AccordionContent>
        </AccordionItem>
      ))}
    </Accordion>
  );
}
