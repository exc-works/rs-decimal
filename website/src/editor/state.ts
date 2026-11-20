import { reactive } from "vue";
type DecimalRow = { value: string; coefficient: string; scale: string };
export const session = reactive({
  initialized: false,
  source: "",
  output: [] as (string | DecimalRow)[],
  running: false,
  status: "idle",
  message: "",
  kinds: [] as string[],
  elapsed: 0,
});
