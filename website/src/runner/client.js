import { SCHEMA, LIMITS, sourceValid, eventValid } from "./protocol.js";

export class RunnerClient {
  constructor({ container, runnerUrl, commit, timeoutMs = LIMITS.timeoutMs }) {
    const url = new URL(runnerUrl);
    const local = ["127.0.0.1", "localhost", "[::1]"].includes(url.hostname);
    if (
      url.origin === location.origin ||
      url.username ||
      url.password ||
      (url.protocol !== "https:" && !(local && url.protocol === "http:"))
    )
      throw new Error("Runner requires a separate trusted static origin");
    if (
      !Number.isInteger(timeoutMs) ||
      timeoutMs < 1 ||
      timeoutMs > LIMITS.timeoutMs
    )
      throw new Error("Invalid deadline");
    this.epoch = 0;
    this.active = null;
    this.disposed = false;
    this.timeoutMs = timeoutMs;
    this.frame = document.createElement("iframe");
    this.frame.hidden = true;
    this.frame.setAttribute("sandbox", "allow-scripts allow-same-origin");
    this.frame.referrerPolicy = "no-referrer";
    const channel = new MessageChannel();
    this.port = channel.port1;
    this.ready = new Promise((resolve, reject) => {
      this.rejectReady = reject;
      this.handshakeTimer = setTimeout(
        () => reject(new Error("Runner handshake failed")),
        8000,
      );
      this.port.onmessage = ({ data }) => {
        if (this.disposed) return;
        if (
          data?.type === "ready" &&
          data.schema === SCHEMA &&
          data.commit === commit
        ) {
          clearTimeout(this.handshakeTimer);
          resolve();
          return;
        }
        const run = this.active;
        if (!run || !eventValid(data, run.id)) return;
        if (data.type === "output") {
          if (
            ++run.count > LIMITS.outputCount ||
            (run.bytes += new TextEncoder().encode(data.text).length) >
              LIMITS.outputBytes
          ) {
            this.stop("resource");
            return;
          }
          try {
            run.onOutput(data.text);
          } catch {
            this.stop("runtime");
          }
        } else this.finish(data);
      };
      this.frame.onload = () => {
        this.frame.onload = null;
        this.frame.contentWindow.postMessage(
          { type: "connect", schema: SCHEMA, commit },
          url.origin,
          [channel.port2],
        );
      };
    });
    this.frame.src = url.href;
    container.append(this.frame);
  }
  async run(source, { onOutput = () => {} } = {}) {
    if (!sourceValid(source))
      throw new Error("Source exceeds 65536 UTF-8 bytes");
    this.stop();
    const id = ++this.epoch;
    let cancel;
    const cancellation = new Promise((resolve) => {
      cancel = resolve;
    });
    this.pending = { id, cancel };
    let ready;
    try {
      ready = await Promise.race([
        this.ready.then(() => true),
        cancellation.then(() => false),
      ]);
    } finally {
      if (this.pending?.id === id) this.pending = null;
    }
    if (!ready || this.disposed || id !== this.epoch)
      return {
        type: "error",
        runId: id,
        category: "cancelled",
        message: "Run cancelled",
      };
    return new Promise((resolve) => {
      this.active = {
        id,
        resolve,
        onOutput,
        count: 0,
        bytes: 0,
        timer: setTimeout(() => {
          if (this.active?.id === id) this.stop("timeout");
        }, this.timeoutMs),
      };
      this.port.postMessage({ type: "run", runId: id, source });
    });
  }
  finish(result) {
    const run = this.active;
    this.active = null;
    if (!run) return;
    clearTimeout(run.timer);
    run.resolve(result);
  }
  stop(category = "cancelled") {
    ++this.epoch;
    const pending = this.pending;
    this.pending = null;
    pending?.cancel();
    const run = this.active;
    if (!run) return;
    this.finish({
      type: "error",
      runId: run.id,
      category,
      message:
        category === "timeout" ? "Execution deadline exceeded" : "Run stopped",
    });
    this.port.postMessage({ type: "stop", runId: run.id });
  }
  dispose() {
    this.stop();
    this.disposed = true;
    clearTimeout(this.handshakeTimer);
    this.rejectReady(new Error("Runner disposed"));
    this.port.close();
    this.frame.remove();
  }
}
