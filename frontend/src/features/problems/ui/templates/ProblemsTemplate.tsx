import { Problem } from "@/shared/model/problem";
import { ContestAccordion } from "../organisis/ContestAccordion";
import { css } from "styled-system/css";

export const ProblemsTemplate = ({
  problemsMap,
}: {
  problemsMap: Map<string, Problem[]>;
}) => {
  return (
    // <div className="container mx-auto p-4">
    <div className={css({ mx: "auto", maxWidth: "8xl" })}>
      <ContestAccordion problemsMap={problemsMap} />
    </div>
  );
};
