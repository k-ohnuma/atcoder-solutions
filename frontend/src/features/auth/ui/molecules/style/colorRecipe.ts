import { cva } from "styled-system/css";

export const dotClass = cva({
  base: { h: 3, w: 3, rounded: "full" },
  variants: {
    color: {
      red: { bg: "red" },
      orange: { bg: "ac.orange" },
      yellow: { bg: "ac.yellow" },
      blue: { bg: "ac.blue" },
      cyan: { bg: "ac.cyan" },
      green: { bg: "ac.green" },
      brown: { bg: "ac.brown" },
      gray: { bg: "ac.gray" },
    },
  },
  defaultVariants: { color: "gray" },
});

export const toggleItemClass = cva({
  base: {
    gap: 2,
    display: "flex",
    alignItems: "center",
    rounded: "md",
    borderWidth: "1px",
    px: "3",
    py: "2",
    cursor: "pointer",
  },
  variants: {
    color: {
      red: {
        "&[data-state=on]": { backgroundColor: "ac.red/70", color: "white" },
      },
      orange: {
        "&[data-state=on]": { backgroundColor: "ac.orange/70", color: "white" },
      },
      yellow: {
        "&[data-state=on]": { backgroundColor: "ac.yellow/70", color: "white" },
      },
      blue: {
        "&[data-state=on]": { backgroundColor: "ac.blue/70", color: "white" },
      },
      cyan: {
        "&[data-state=on]": { backgroundColor: "ac.cyan/70", color: "white" },
      },
      green: {
        "&[data-state=on]": { backgroundColor: "ac.green/70", color: "white" },
      },
      brown: {
        "&[data-state=on]": { backgroundColor: "ac.brown/70", color: "white" },
      },
      gray: {
        "&[data-state=on]": { backgroundColor: "ac.gray/70", color: "white" },
      },
    },
  },
  defaultVariants: { color: "gray" },
});
