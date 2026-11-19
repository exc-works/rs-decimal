import { reactive } from "vue";
export const session = reactive({
  initialized: false,
  source: "",
  output: [] as string[],
  running: false,
  status: "idle",
  message: "",
  kinds: [] as string[],
  elapsed: 0,
});
