import { defineConfig } from "@pandacss/dev";
import presetBase from "@pandacss/preset-base";
import presetPanda from "@pandacss/preset-panda";
import { createPreset } from "@park-ui/panda-preset";
import amber from "@park-ui/panda-preset/colors/amber";
import sand from "@park-ui/panda-preset/colors/sand";

export default defineConfig({
  // Whether to use css reset
  preflight: true,

  presets: [presetBase, presetPanda, createPreset({ accentColor: amber, grayColor: sand, radius: "sm" })],

  // Where to look for your css declarations
  include: ["./src/**/*.{js,jsx,ts,tsx}", "./pages/**/*.{js,jsx,ts,tsx}"],

  // Files to exclude
  exclude: [],

  // Useful for theme customization
  theme: {
    extend: {
      semanticTokens: {
        colors: {
          "ac.gray": {
            value: { base: "#808080" },
          },
          "ac.brown": {
            value: { base: "#804000" },
          },
          "ac.green": {
            value: { base: "#008000" },
          },
          "ac.cyan": {
            value: { base: "#00C0C0" },
          },
          "ac.blue": {
            value: { base: "#0000FF" },
          },
          "ac.yellow": {
            value: { base: "#C0C000" },
          },
          "ac.orange": {
            value: { base: "#FF8000" },
          },
          "ac.red": {
            value: { base: "#FF0000" },
          },
        },
      },
    },
  },

  // The output directory for your css system
  outdir: "styled-system",
  jsxFramework: "react",
});
