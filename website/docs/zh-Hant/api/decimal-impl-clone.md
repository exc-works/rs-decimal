---
locale: zh-Hant
source_revision: 37fcbc194399d8059373fc9c0c61833d997ffb41871435eb1c6867b7870d1acc
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::impl Clone
---
# Decimal::impl Clone

複製 Decimal 並保留完整表示。

```rust
impl Clone for Decimal
```

## 常規用法及語義

Clone 複製係數、小數位數及初始化狀態；JavaScript 使用具名 clone() 適配器，不提供 Rust trait 語法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-clone" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
