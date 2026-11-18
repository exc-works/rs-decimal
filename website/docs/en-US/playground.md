---
locale: en-US
source_revision: a8248cf86b647c5ae7b7b6b6d557a514ea0e7a1ff72bdd8a4e20a36c0572eb5a
---
# Playground

Write JavaScript using Rust method names. Run explicitly to inspect exact values and state.

<script setup>
import Playground from "../_components/Playground.vue";
import { locales } from "../../locales.mjs";
</script>

<Playground locale="en-US" :labels="locales['en-US']" />
