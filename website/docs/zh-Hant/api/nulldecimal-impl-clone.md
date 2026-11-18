---
locale: zh-Hant
source_revision: 056afbd5297b8421283bf4270ff73e28ea9129becd2598b31ff7a84ee04dacd5
translation_provenance: zh-Hans-terminology-reviewed
api_id: NullDecimal::impl Clone
---
# NullDecimal::impl Clone

在 Rust 中同時複製有效性旗標及內部 Decimal。

```rust
impl Clone for NullDecimal
```

## 常規用法及語義

即使 valid 為 false，複製也保留內部值。這是原生 Rust trait，不是即時共享參照。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-clone" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
