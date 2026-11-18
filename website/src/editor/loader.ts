let loadedLanguage: string | undefined;
const languages: Record<string, () => Promise<unknown>> = {
  "zh-Hans": () => import("monaco-editor/nls/lang/zh-cn"),
  "zh-Hant": () => import("monaco-editor/nls/lang/zh-tw"),
  fr: () => import("monaco-editor/nls/lang/fr"),
  ko: () => import("monaco-editor/nls/lang/ko"),
  ja: () => import("monaco-editor/nls/lang/ja"),
  de: () => import("monaco-editor/nls/lang/de"),
};
export async function loadEditor(locale: string) {
  const language = locale.startsWith("en-") ? "en" : locale;
  if (loadedLanguage && loadedLanguage !== language) {
    // Monaco captures some translated labels during module initialization.
    // Draft/result state is saved before locale navigation; reload retains it.
    location.reload();
    return;
  }
  if (!loadedLanguage) {
    loadedLanguage = language;
    await languages[locale]?.();
  }
  return import("./monaco");
}
