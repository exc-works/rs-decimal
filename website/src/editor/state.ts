import { reactive } from "vue";
import { DEFAULT_SOURCE } from "./draft.mjs";
export const session = reactive({
  initialized: false,
  source: DEFAULT_SOURCE,
  output: [] as string[],
  running: false,
  status: "idle",
  message: "",
  kinds: [] as string[],
  elapsed: 0,
});
