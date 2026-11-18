export const SCHEMA = 1;
export const LIMITS = Object.freeze({
  sourceBytes: 65536,
  outputBytes: 262144,
  outputCount: 100,
  timeoutMs: 3000,
});
export function sourceValid(source) {
  return (
    typeof source === "string" &&
    source.length <= LIMITS.sourceBytes &&
    new TextEncoder().encode(source).length <= LIMITS.sourceBytes
  );
}
export function eventValid(data, runId) {
  return (
    data &&
    data.runId === runId &&
    ["output", "done", "error"].includes(data.type) &&
    (data.type !== "output" ||
      (typeof data.text === "string" &&
        data.text.length <= LIMITS.outputBytes)) &&
    (data.type !== "error" ||
      (typeof data.category === "string" &&
        data.category.length <= 32 &&
        typeof data.message === "string" &&
        data.message.length <= 8192))
  );
}
