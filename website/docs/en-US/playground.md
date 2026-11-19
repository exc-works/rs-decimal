---
locale: en-US
source_revision: 284f92d20227e82946bbbcda123bebd9143e64512e1a7f0633cf17755157f389
---
# Playground

Run the receipt for two notebooks at 4.50 each and 1.25 delivery. Read the amount due (10.25) and the change from 20.00 (9.75). Then change the quantity from `2n` to `3n` and run again. The new total is 14.75.

If your own draft is open, export it before selecting **Reset example** to load the receipt.

<script setup>
import Playground from "../_components/Playground.vue";
import { locales } from "../../locales.mjs";
</script>

<Playground locale="en-US" :labels="locales['en-US']" />

::: details Method examples and sharing
Edit an argument on a method page to update the example and its result. Open it in Playground to edit or share the full script.

Share creates a URL fragment containing up to 4 KiB of UTF-8 code and the library build commit. The code is not uploaded to a server. A shared link requires the same build, loads without running, and clears previous results. Invalid links leave your current draft intact. Review the code and click Run. Export larger scripts as files; the editor accepts up to 64 KiB. Links can contain any text in your script, so inspect the code before sharing.
:::

[Playground resource policy](./resource-policy)
