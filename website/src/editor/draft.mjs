import { sourceValid, LIMITS } from "../runner/protocol.js";
export const DRAFT_KEY = "rs-decimal:playground:source:v1";
export const RESULT_KEY = "rs-decimal:playground:result:v1";
export function decodeDraft(text) {
  if (typeof text !== "string" || text.length > LIMITS.sourceBytes * 6 + 256)
    throw Error("Invalid draft");
  const data = JSON.parse(text);
  if (data?.schema !== 1 || !sourceValid(data.source))
    throw Error("Invalid draft");
  return data.source;
}
export function encodeDraft(source) {
  if (!sourceValid(source)) throw Error("Invalid draft");
  return JSON.stringify({ schema: 1, source });
}
export function decodeResult(text) {
  if (typeof text !== "string" || text.length > LIMITS.outputBytes * 6 + 65536)
    throw Error("Invalid result");
  const data = JSON.parse(text);
  if (
    data?.schema !== 1 ||
    !Array.isArray(data.output) ||
    data.output.length > LIMITS.outputCount ||
    data.output.some((x) => typeof x !== "string") ||
    new TextEncoder().encode(data.output.join("")).length > LIMITS.outputBytes
  )
    throw Error("Invalid result");
  const status = data.status ?? "idle",
    message = data.message ?? "",
    kinds = data.kinds ?? [],
    elapsed = data.elapsed ?? 0;
  if (
    typeof status !== "string" ||
    status.length > 32 ||
    typeof message !== "string" ||
    message.length > 8192 ||
    !Array.isArray(kinds) ||
    kinds.length > 16 ||
    kinds.some((x) => typeof x !== "string" || x.length > 64) ||
    !Number.isFinite(elapsed) ||
    elapsed < 0 ||
    elapsed > 60000
  )
    throw Error("Invalid result");
  return { output: data.output, status, message, kinds, elapsed };
}

export function encodeResult(state) {
  const text = JSON.stringify({
    schema: 1,
    output: state.output,
    status: state.status,
    message: state.message,
    kinds: state.kinds,
    elapsed: state.elapsed,
  });
  decodeResult(text);
  return text;
}
