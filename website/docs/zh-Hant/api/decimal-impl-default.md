---
locale: zh-Hant
source_revision: 9ab9d90f4fdbf97aeb5b036e4b2c138345a66dab28af2a920054edaddf0608dd
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::impl Default
---
# Decimal::impl Default

透過 Rust Default 建立未初始化的數值零。

```rust
impl Default for Decimal
```

## 常規用法及語義

預設值在數值上與建構的零相等，但 is_nil() 為 true。bridge 提供 Decimal.default()；此頁描述的是 Rust trait 本身。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-default" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
