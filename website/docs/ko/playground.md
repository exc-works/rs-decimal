---
locale: ko
source_revision: a8248cf86b647c5ae7b7b6b6d557a514ea0e7a1ff72bdd8a4e20a36c0572eb5a
---
# 플레이그라운드

JavaScript에서 Rust와 같은 이름의 메서드를 호출하세요. 직접 실행하여 정확한 값과 상태를 확인합니다.

<script setup>
import Playground from "../_components/Playground.vue";
import { locales } from "../../locales.mjs";
</script>

<Playground locale="ko" :labels="locales['ko']" />
