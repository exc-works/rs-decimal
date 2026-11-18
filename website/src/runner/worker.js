import init, * as raw from "../../public/wasm/decimal.js";
import wasmBytes from "../../public/wasm/decimal_bg.wasm";
import { createBridge } from "../bridge/index.js";
import { LIMITS } from "./protocol.js";

const config = __RUNNER_CONFIG__;
const stringify = JSON.stringify.bind(JSON);
const descriptors = Object.getOwnPropertyDescriptors;
const entries = Object.entries;
const isArray = Array.isArray;
const closeWorker = self.close.bind(self);
const encode = new TextEncoder().encode.bind(new TextEncoder());
const runFunction = Object.getPrototypeOf(async function () {}).constructor;

self.onmessage = async ({ data, ports }) => {
  self.onmessage = null; // one bootstrap and one fresh Wasm instance per Worker
  const port = ports[0];
  if (!port || !data || typeof data.source !== "string") {
    closeWorker();
    return;
  }
  const send = port.postMessage.bind(port);
  const runId = data.runId;
  let outputCount = 0,
    outputBytes = 0;
  try {
    // These probes must be blocked by the response CSP before any user code runs.
    // The static package supplies successful probe endpoints for missing-CSP tests.
    let networkOpen = false;
    try {
      await fetch(new URL("./probe.js", import.meta.url));
      networkOpen = true;
    } catch {}
    let importsOpen = false;
    try {
      await import(
        /* @vite-ignore */ new URL("./probe.js", import.meta.url).href
      );
      importsOpen = true;
    } catch {}
    const workersOpen = await new Promise((resolve) => {
      try {
        const probe = new Worker(new URL("./probe-worker.js", import.meta.url));
        probe.onmessage = () => {
          probe.terminate();
          resolve(true);
        };
        probe.onerror = (event) => {
          event.preventDefault();
          probe.terminate();
          resolve(false);
        };
      } catch {
        resolve(false);
      }
    });
    if (networkOpen || importsOpen || workersOpen)
      throw Object.assign(new Error("Runner CSP isolation is missing"), {
        category: "isolation",
      });
    await init({ module_or_path: wasmBytes });
    const bridge = createBridge(raw);
    if (bridge.metadata().commit !== config.commit)
      throw Object.assign(new Error("Runner/Wasm version mismatch"), {
        category: "version",
      });
    const snapshot = bridge.snapshot;
    function print(value) {
      let nodes = 0;
      const seen = new WeakSet();
      function bounded(input, depth = 0) {
        if (++nodes > 1000 || depth > 4) return "[truncated]";
        if (typeof input === "string") return input.slice(0, 8192);
        if (typeof input === "bigint")
          return input.toString().slice(0, 8192) + "n";
        if (
          input === null ||
          typeof input === "number" ||
          typeof input === "boolean"
        )
          return input;
        if (typeof input !== "object") return `[${typeof input}]`;
        if (seen.has(input)) return "[circular]";
        seen.add(input);
        if (
          input instanceof bridge.Decimal ||
          input instanceof bridge.NullDecimal
        )
          return bounded(snapshot(input), depth + 1);
        const out = isArray(input) ? [] : {};
        for (const [key, descriptor] of entries(descriptors(input)).slice(
          0,
          100,
        )) {
          if (key === "length" || key === "__proto__") continue;
          out[key.slice(0, 128)] =
            "value" in descriptor
              ? bounded(descriptor.value, depth + 1)
              : "[accessor]";
        }
        return out;
      }
      const text = stringify(bounded(value));
      const bytes = encode(text).length;
      if (
        ++outputCount > LIMITS.outputCount ||
        (outputBytes += bytes) > LIMITS.outputBytes
      )
        throw Object.assign(new Error("Output budget exceeded"), {
          category: "resource",
        });
      send({ type: "output", runId, text });
    }
    await new runFunction(
      "Decimal",
      "NullDecimal",
      "ByteBuffer",
      "RoundingMode",
      "snapshot",
      "zero",
      "one",
      "ten",
      "hundred",
      "min",
      "max",
      "between",
      "print",
      "show",
      "assert",
      '"use strict";\n' + data.source,
    )(
      bridge.Decimal,
      bridge.NullDecimal,
      bridge.ByteBuffer,
      bridge.RoundingMode,
      snapshot,
      bridge.zero,
      bridge.one,
      bridge.ten,
      bridge.hundred,
      bridge.min,
      bridge.max,
      bridge.between,
      print,
      print,
      (condition, message = "Assertion failed") => {
        if (!condition)
          throw Object.assign(new Error(String(message).slice(0, 8192)), {
            category: "assertion",
          });
      },
    );
    send({ type: "done", runId });
  } catch (error) {
    // Diagnostics are bounded strings; no user HTML or thrown object crosses the port.
    let message = "Execution failed",
      category = "runtime";
    let kinds = [];
    try {
      message = String(error?.message ?? error).slice(0, 8192);
      category = String(
        error?.category ??
          (error instanceof WebAssembly.RuntimeError ? "trap" : "runtime"),
      ).slice(0, 32);
      if (isArray(error?.kinds))
        kinds = error.kinds
          .slice(0, 16)
          .map((kind) => String(kind).slice(0, 64));
    } catch {}
    send({ type: "error", runId, category, message, kinds });
  } finally {
    port.close();
    closeWorker();
  }
};
