---
locale: zh-Hant
source_revision: 64d94a480e185cbb38c975ef60712d5ae4ac373961962b675392db6ef9375f09
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::remainder
---
# Decimal::remainder

拆分整數部分和未縮放的小數餘數。

```rust
fn remainder (& self) -> (BigInt , BigInt)
```

## 參數、回傳值及常規用法

無參數；回傳 (BigInt, BigInt)，映射為 [bigint, bigint]。第一項向零截斷；第二項是帶符號的係數餘數，不是 Decimal 小數。-12.750 得到 (-12, -750)：第二項需要原始 scale 3 才表示 -0.750。預設零回傳 (0, 0)。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-remainder" locale="zh-Hant" />

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
