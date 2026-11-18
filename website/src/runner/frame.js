import { SCHEMA, LIMITS, sourceValid, eventValid } from "./protocol.js";
const config = __RUNNER_CONFIG__;
let connected = false;
window.addEventListener("message", (event) => {
  if (
    connected ||
    event.source !== parent ||
    !config.parentOrigins.includes(event.origin) ||
    event.data?.type !== "connect" ||
    event.data.schema !== SCHEMA ||
    event.data.commit !== config.commit ||
    event.ports.length !== 1
  )
    return;
  connected = true;
  const port = event.ports[0];
  let active = null;
  function finish(type, extra = {}) {
    if (!active) return;
    const previous = active;
    active = null; // invalidate before terminating
    clearTimeout(previous.timer);
    previous.channel.close();
    previous.worker.terminate();
    port.postMessage({ type, runId: previous.id, ...extra });
  }
  port.onmessage = ({ data }) => {
    if (data?.type === "stop" && active?.id === data.runId) {
      finish("error", { category: "cancelled", message: "Run stopped" });
      return;
    }
    if (
      data?.type !== "run" ||
      !Number.isSafeInteger(data.runId) ||
      data.runId < 1 ||
      !sourceValid(data.source)
    )
      return;
    if (active)
      finish("error", { category: "cancelled", message: "Run replaced" });
    const worker = new Worker(new URL("./worker.js", import.meta.url), {
      type: "module",
    });
    const channel = new MessageChannel();
    const run = {
      id: data.runId,
      worker,
      channel: channel.port1,
      count: 0,
      bytes: 0,
    };
    active = run;
    run.timer = setTimeout(() => {
      if (active === run)
        finish("error", {
          category: "timeout",
          message: "Execution deadline exceeded",
        });
    }, LIMITS.timeoutMs);
    worker.onerror = () => {
      if (active === run)
        finish("error", { category: "runtime", message: "Worker failed" });
    };
    // Ignore global worker messages; only the private bootstrap port is trusted.
    channel.port1.onmessage = ({ data: message }) => {
      if (active !== run || !eventValid(message, run.id)) return;
      if (message.type === "output") {
        if (
          ++run.count > LIMITS.outputCount ||
          (run.bytes += new TextEncoder().encode(message.text).length) >
            LIMITS.outputBytes
        ) {
          finish("error", {
            category: "resource",
            message: "Output budget exceeded",
          });
          return;
        }
        port.postMessage(message);
      } else
        finish(
          message.type,
          message.type === "error"
            ? {
                category: message.category,
                message: message.message,
                kinds: message.kinds,
              }
            : {},
        );
    };
    worker.postMessage({ source: data.source, runId: run.id }, [channel.port2]);
  };
  port.postMessage({ type: "ready", schema: SCHEMA, commit: config.commit });
});
