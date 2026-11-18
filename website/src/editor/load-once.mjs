// Every caller must await locale initialization before importing Monaco.
export function createEditorLoader(languages, importEditor, reload) {
  let selectedLanguage, pending;
  return (locale) => {
    const language = locale.startsWith("en-") ? "en" : locale;
    if (selectedLanguage && selectedLanguage !== language) {
      reload();
      return Promise.resolve(undefined);
    }
    if (!pending) {
      selectedLanguage = language;
      pending = Promise.resolve().then(() => languages[locale]?.()).then(importEditor);
    }
    return pending;
  };
}
