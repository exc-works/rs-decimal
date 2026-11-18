---
locale: en-US
source_revision: fbadb341fac6ded3986e830a05797cf96857f33d92f3224a113dd3d05ba9eb85
---
# Playground

Write JavaScript using Rust method names. Run explicitly to inspect exact values and state.

<script setup>
import Playground from "../_components/Playground.vue";
import { locales } from "../../locales.mjs";
</script>

<Playground locale="en-US" :labels="locales['en-US']" />

Method pages provide forms for literal arguments in their shared examples. Parameters follow JavaScript call order; Rust method names and argument order are preserved. The generated preview opens in this editor for complex values and additional calls. The expected state on the method page describes the original example. Form edits survive language changes in this tab.

Share creates a URL fragment containing up to 4 KiB of UTF-8 code and the library build commit. The code is not uploaded to a server. A shared link requires the same build, loads without running, and clears previous results. Invalid links leave your current draft intact. Review the code and click Run. Export larger scripts as files; the editor accepts up to 64 KiB. Links can contain any text in your script, so inspect the code before sharing.
