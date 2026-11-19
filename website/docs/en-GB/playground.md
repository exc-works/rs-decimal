---
locale: en-GB
source_revision: 60b71a4e750d3835abdec3e2e546f11a77759013f3f84ad31d530c433bd67b11
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
Run examples on the method page to try different arguments. Open one in Playground to change the script or share it.

Share creates a URL fragment containing up to 4 KiB of UTF-8 code and the library build commit. The code is not uploaded to a server. A shared link requires the same build, loads without running, and clears previous results. Invalid links leave your current draft intact. Review the code and click Run. Export larger scripts as files; the editor accepts up to 64 KiB. Links can contain any text in your script, so inspect the code before sharing.
:::

[Playground resource policy](./resource-policy)
