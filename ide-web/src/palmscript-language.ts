import type { Monaco } from "@monaco-editor/react";

let configured = false;

const KEYWORDS = [
  "and",
  "const",
  "else",
  "entry",
  "exit",
  "export",
  "false",
  "fn",
  "if",
  "input",
  "interval",
  "let",
  "na",
  "order",
  "plot",
  "protect",
  "source",
  "target",
  "trigger",
  "true",
  "use",
];

const BUILTINS = [
  "above",
  "atr",
  "below",
  "coalesce",
  "crossover",
  "ema",
  "highest",
  "highest_since",
  "kama",
  "lowest",
  "macd",
  "plot",
  "risk_pct",
  "rsi",
  "sma",
];

export function configurePalmScriptLanguage(monaco: Monaco): void {
  if (configured) {
    return;
  }
  configured = true;

  monaco.languages.register({ id: "palmscript" });
  monaco.languages.setLanguageConfiguration("palmscript", {
    comments: {
      lineComment: "//",
    },
    brackets: [
      ["{", "}"],
      ["[", "]"],
      ["(", ")"],
    ],
    autoClosingPairs: [
      { open: "{", close: "}" },
      { open: "[", close: "]" },
      { open: "(", close: ")" },
      { open: '"', close: '"' },
    ],
  });

  monaco.languages.setMonarchTokensProvider("palmscript", {
    keywords: KEYWORDS,
    builtins: BUILTINS,
    tokenizer: {
      root: [
        [/\/\/.*$/, "comment"],
        [/"[^"]*"/, "string"],
        [/\b\d+(\.\d+)?\b/, "number"],
        [/[{}()[\]]/, "@brackets"],
        [
          /[a-zA-Z_][\w.]*/,
          {
            cases: {
              "@keywords": "keyword",
              "@builtins": "predefined",
              "@default": "identifier",
            },
          },
        ],
        [/[=><!~?:&|+\-*/^%]+/, "operator"],
      ],
    },
  });

  monaco.editor.defineTheme("palmscript-docs", {
    base: "vs",
    inherit: true,
    rules: [
      { token: "keyword", foreground: "0f5d92", fontStyle: "bold" },
      { token: "predefined", foreground: "1b87d6" },
      { token: "identifier", foreground: "1f3142" },
      { token: "number", foreground: "9c4f14" },
      { token: "string", foreground: "1a7f5a" },
      { token: "comment", foreground: "718598" },
      { token: "operator", foreground: "425466" },
    ],
    colors: {
      "editor.background": "#f6f9fc",
      "editor.foreground": "#173246",
      "editor.lineHighlightBackground": "#eef5fb",
      "editorLineNumber.foreground": "#98aabd",
      "editorCursor.foreground": "#1f8de1",
      "editor.selectionBackground": "#cfe4f7",
      "editor.inactiveSelectionBackground": "#dbeaf6",
    },
  });
}
