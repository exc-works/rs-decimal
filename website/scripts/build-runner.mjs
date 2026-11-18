import { build } from "esbuild";
import { readFile, mkdir, writeFile, copyFile, rm } from "node:fs/promises";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
const root = fileURLToPath(new URL("../", import.meta.url));
const origins = JSON.parse(process.env.RUNNER_PARENT_ORIGINS || "[]");
if (!Array.isArray(origins) || !origins.length)
  throw Error("Set RUNNER_PARENT_ORIGINS to an explicit JSON origin allowlist");
for (const origin of origins) {
  const url = new URL(origin);
  if (
    origin !== url.origin ||
    (url.protocol !== "https:" &&
      !(
        process.env.RUNNER_ALLOW_LOCALHOST === "1" &&
        url.protocol === "http:" &&
        ["127.0.0.1", "localhost", "[::1]"].includes(url.hostname)
      ))
  )
    throw Error("Invalid parent origin");
}
const commit = execFileSync("git", ["rev-parse", "HEAD"], {
  cwd: root,
  encoding: "utf8",
}).trim();
const wasm = JSON.parse(
  await readFile(new URL("../public/wasm/build.json", import.meta.url), "utf8"),
);
if (wasm.commit !== commit || wasm.wasm_memory_limit_bytes !== 67108864)
  throw Error(
    "Rebuild bounded Wasm at the current commit before packaging runner",
  );
const config = { commit, parentOrigins: origins };
const out = new URL("../runner-dist/", import.meta.url);
await rm(out, { recursive: true, force: true });
await mkdir(out, { recursive: true });
await build({
  absWorkingDir: root,
  entryPoints: ["src/runner/frame.js", "src/runner/worker.js"],
  outdir: fileURLToPath(out),
  bundle: true,
  format: "esm",
  platform: "browser",
  target: "es2022",
  loader: { ".wasm": "binary" },
  define: { __RUNNER_CONFIG__: JSON.stringify(config) },
});
await copyFile(
  new URL("../runner/index.html", import.meta.url),
  new URL("index.html", out),
);
await writeFile(new URL("probe.js", out), "export const probe = true;\n");
await writeFile(new URL("probe-worker.js", out), 'postMessage("open");\n');
const frameCsp = `default-src 'none'; script-src 'self'; worker-src 'self'; connect-src 'none'; frame-ancestors ${origins.join(" ")}; base-uri 'none'; form-action 'none'; object-src 'none'`;
// CSP paths do not overlap: hosts that append duplicate headers must not apply
// the frame's script-src to the worker's eval-enabled, network-disabled policy.
const headers = {
  "/*": {
    "X-Content-Type-Options": "nosniff",
    "Referrer-Policy": "no-referrer",
    "Cache-Control": "no-store",
  },
  "/": { "Content-Security-Policy": frameCsp },
  "/index.html": { "Content-Security-Policy": frameCsp },
  "/worker.js": {
    "Content-Security-Policy":
      "default-src 'none'; script-src 'unsafe-eval'; connect-src 'none'; worker-src 'none'; base-uri 'none'; object-src 'none'",
  },
};
await writeFile(
  new URL("headers.json", out),
  JSON.stringify(headers, null, 2) + "\n",
);
await writeFile(
  new URL("_headers", out),
  Object.entries(headers)
    .map(
      ([path, values]) =>
        path +
        "\n" +
        Object.entries(values)
          .map(([k, v]) => `  ${k}: ${v}`)
          .join("\n"),
    )
    .join("\n") + "\n",
);
await writeFile(
  new URL("manifest.json", out),
  JSON.stringify({ schema: 1, ...config }, null, 2) + "\n",
);
console.log(
  `Built isolated runner at ${commit}; deploy with response headers on a separate static origin.`,
);
