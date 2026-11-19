---
locale: en-GB
source_revision: eebc95321692656eeaf68196d3bb7fece103772431d4688264c85fec9c739db3
---
# Playground

Run the receipt for two notebooks at 4.50 each and 1.25 delivery. Read the amount due (10.25) and the change from 20.00 (9.75). Then change the quantity from `2n` to `3n` and run again. The new total is 14.75.

If your own draft is open, export it before selecting **Reset example** to load the receipt.

<script setup>
import Playground from "../_components/Playground.vue";
import { locales } from "../../locales.mjs";
</script>

<Playground locale="en-GB" :labels="locales['en-GB']" />

::: details Method examples and sharing
Method pages provide forms for literal arguments in their shared examples. Parameters follow JavaScript call order; Rust method names and argument order are preserved. The generated preview opens in this editor for complex values and additional calls. The expected state on the method page describes the original example. Form edits survive language changes in this tab.

Share creates a URL fragment containing up to 4 KiB of UTF-8 code and the library build commit. The code is not uploaded to a server. A shared link requires the same build, loads without running, and clears previous results. Invalid links leave your current draft intact. Review the code and click Run. Export larger scripts as files; the editor accepts up to 64 KiB. Links can contain any text in your script, so inspect the code before sharing.
:::

[Playground resource policy](./resource-policy)
