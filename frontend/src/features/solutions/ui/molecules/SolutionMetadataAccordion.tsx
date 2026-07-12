import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from "@/components/ui/accordion";
import { SolutionMetadataTable } from "./SolutionMetadataTable";
import { SolutionOwnerActions } from "./SolutionOwnerActions";

type SolutionMetadataAccordionProps = {
  solutionId: string;
  problemId: string;
  ownerUserId: string;
  title: string;
  bodyMd: string;
  submitUrl: string;
  tags: string[];
  userName: string;
  createdAt: string;
  atcoderProblemUrl: string;
};

export function SolutionMetadataAccordion({
  solutionId,
  problemId,
  ownerUserId,
  title,
  bodyMd,
  submitUrl,
  tags,
  userName,
  createdAt,
  atcoderProblemUrl,
}: SolutionMetadataAccordionProps) {
  return (
    <Accordion type="single" collapsible className="mt-3">
      <AccordionItem value="meta" className="border-b-0">
        <AccordionTrigger className="justify-end py-2">
          <span className="sr-only">詳細情報</span>
        </AccordionTrigger>
        <AccordionContent className="pb-1">
          <SolutionMetadataTable
            userName={userName}
            createdAt={createdAt}
            atcoderProblemUrl={atcoderProblemUrl}
            submitUrl={submitUrl}
            tags={tags}
          />
          <div className="mt-3">
            <SolutionOwnerActions
              solutionId={solutionId}
              problemId={problemId}
              ownerUserId={ownerUserId}
              title={title}
              bodyMd={bodyMd}
              submitUrl={submitUrl}
              tags={tags}
            />
          </div>
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  );
}
