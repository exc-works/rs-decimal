---
locale: zh-Hant
source_revision: 25b1ec5a4bb0557629fac6d66ca48c015418489e6280f42443d66425e8372440
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::impl PartialOrd
---
# Decimal::impl PartialOrd

透過 Rust 的偏序比較 Decimal。

```rust
impl PartialOrd for Decimal
```

## 常規用法及語義

所有 Decimal 值都可比較，因此 partial_cmp 回傳 Some(Ordering)。預設零可與建構的零比較，小數位數不同不會產生 None。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialord" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
