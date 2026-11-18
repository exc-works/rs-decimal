---
locale: zh-Hant
source_revision: 1a22225e09dfc3b3b1f7504680c33552c2bd9a622eca096fdf946a5ba9c5cfaa
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::impl PartialOrd::partial_cmp
---
# Decimal::impl PartialOrd::partial_cmp

透過 Rust 的偏序比較 Decimal。

```rust
fn partial_cmp (& self , other : & Self) -> Option < Ordering >
```

## 常規用法及語義

所有 Decimal 值都可比較，因此 partial_cmp 回傳 Some(Ordering)。預設零可與建構的零比較，小數位數不同不會產生 None。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialord-partial-cmp" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
