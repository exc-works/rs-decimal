---
locale: en-US
source_revision: b0ed31c9fd5b925f380c834184c84a74d9c454dd7b30cb7cce9e6b371d0d2219
---
# API catalogue

This inventory is extracted from the Rust source, including methods, types, fields, variants, traits and re-exports. The count is not a count of runnable methods. Planned entries are not yet implemented; Rust-only entries require a native usage example rather than browser execution.

<script setup>
import ApiInventory from "../../_components/ApiInventory.vue";
import { locales } from "../../../locales.mjs";
</script>

<ApiInventory :labels="locales['en-US']" />
