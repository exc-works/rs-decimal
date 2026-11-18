import { createEditorLoader } from "./load-once.mjs";
const languages: Record<string, () => Promise<unknown>> = {
  "zh-Hans": () => import("monaco-editor/nls/lang/zh-cn"),
  "zh-Hant": () => import("monaco-editor/nls/lang/zh-tw"),
  fr: () => import("monaco-editor/nls/lang/fr"),
  ko: () => import("monaco-editor/nls/lang/ko"),
  ja: () => import("monaco-editor/nls/lang/ja"),
  de: () => import("monaco-editor/nls/lang/de"),
};
export const loadEditor = createEditorLoader(
  languages,
  () => import("./monaco"),
  () => location.reload(),
);
