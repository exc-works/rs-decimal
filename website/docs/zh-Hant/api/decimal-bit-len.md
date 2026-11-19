---
locale: zh-Hant
source_revision: cd78c2dcf4b67b04ef802fd73ba3ef0da02aad24d1bb17be5cc5df5d221df3cd
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::bit_len
---
# Decimal::bit_len

統計未縮放係數絕對值的位元數。

```rust
fn bit_len (& self) -> usize
```

## 參數、回傳值及常規用法

無參數，Rust 回傳 usize，JavaScript 回傳 number。不應用 scale，也不計符號位：-1.20 的係數為 -120，位數為 7。零（包括預設零）回傳 0。這不是最高置位的索引。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-bit-len" locale="zh-Hant" />

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
