/// <reference types="vite/client" />

declare module "monaco-editor/esm/vs/editor/editor.worker?worker&inline" {
  const MonacoEditorWorker: {
    new (): Worker;
  };

  export default MonacoEditorWorker;
}
