import React from "react";
import ReactDOM from "react-dom/client";
import * as monaco from "monaco-editor";
import { loader } from "@monaco-editor/react";
import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker&inline";

import { App } from "./App";
import "./styles.css";

loader.config({ monaco });

(globalThis as typeof globalThis & {
  MonacoEnvironment?: { getWorker: () => Worker };
}).MonacoEnvironment = {
  getWorker: () => new editorWorker(),
};

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
