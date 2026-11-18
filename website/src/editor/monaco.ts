import * as monaco from "monaco-editor/editor/editor.api";
import "monaco-editor/editor/browser/coreCommands";
import * as ts from "monaco-editor/languages/features/typescript/register";
import "monaco-editor/languages/definitions/javascript/register";
import "monaco-editor/editor/contrib/suggest/browser/suggestController";
import "monaco-editor/editor/contrib/hover/browser/hoverContribution";
import "monaco-editor/editor/contrib/parameterHints/browser/parameterHints";
import "monaco-editor/editor/contrib/find/browser/findController";
import "monaco-editor/editor/contrib/clipboard/browser/clipboard";
import "monaco-editor/editor/contrib/contextmenu/browser/contextmenu";
import "monaco-editor/editor/contrib/tokenization/browser/tokenization";
import EditorWorker from "monaco-editor/editor/editor.worker?worker";
import TypeScriptWorker from "monaco-editor/languages/features/typescript/ts.worker?worker";
import declarations from "../bridge/index.d.ts?raw";

(globalThis as any).MonacoEnvironment = {
  getWorker(_: string, label: string) {
    return label === "typescript" || label === "javascript"
      ? new TypeScriptWorker()
      : new EditorWorker();
  },
};
ts.javascriptDefaults.setCompilerOptions({
  target: ts.ScriptTarget.ES2022,
  allowNonTsExtensions: true,
  allowJs: true,
  checkJs: true,
  lib: ["es2022"],
  noEmit: true,
});
ts.javascriptDefaults.addExtraLib(
  `declare module "decimal-playground" { ${declarations} }`,
  "file:///decimal-playground.d.ts",
);
ts.javascriptDefaults.addExtraLib(
  `import type { Bridge } from "decimal-playground";
declare global {
  const Decimal: Bridge["Decimal"];
  const NullDecimal: Bridge["NullDecimal"];
  const ByteBuffer: Bridge["ByteBuffer"];
  const GoFormat: Bridge["GoFormat"];
  const RoundingMode: Bridge["RoundingMode"];
  const snapshot: Bridge["snapshot"];
  const zero: Bridge["zero"];
  const one: Bridge["one"];
  const ten: Bridge["ten"];
  const hundred: Bridge["hundred"];
  const min: Bridge["min"];
  const max: Bridge["max"];
  const between: Bridge["between"];
  const most_significant_bit: Bridge["most_significant_bit"];
  const PRECISION_FIXED_SIZE: Bridge["PRECISION_FIXED_SIZE"];
  const COMPATIBILITY_REVISION: Bridge["COMPATIBILITY_REVISION"];
  function print(value: unknown): void;
  function show(value: unknown): void;
  function assert(condition: unknown, message?: string): asserts condition;
}`,
  "file:///globals.d.ts",
);

export function createEditor(
  element: HTMLElement,
  source: string,
  dark: boolean,
  label: string,
  run: () => void,
) {
  const model = monaco.editor.createModel(
    source,
    "javascript",
    monaco.Uri.parse("file:///playground.js"),
  );
  const editor = monaco.editor.create(element, {
    model,
    theme: dark ? "vs-dark" : "vs",
    automaticLayout: true,
    minimap: { enabled: false },
    fontSize: 14,
    scrollBeyondLastLine: false,
    ariaLabel: label,
    tabSize: 2,
    wordWrap: "on",
    accessibilitySupport: "auto",
  });
  editor.addAction({
    id: "decimal.run",
    label,
    keybindings: [monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter],
    run,
  });
  return {
    getValue: () => model.getValue(),
    setValue: (value: string) => {
      model.setValue(value);
      editor.setPosition(model.getPositionAt(value.length));
    },
    onChange: (callback: () => void) =>
      editor.onDidChangeModelContent(callback),
    setTheme: (value: boolean) =>
      monaco.editor.setTheme(value ? "vs-dark" : "vs"),
    dispose: () => {
      editor.dispose();
      model.dispose();
    },
  };
}
