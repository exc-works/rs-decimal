---
locale: zh-Hant
source_revision: cfbab45ed5115c58df815ab40afe368434f97862eeaecc53b795f1f752fe2e25
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::impl Eq
---
# Decimal::impl Eq

在 Rust 要求完全相等關係的場景中使用 Decimal。

```rust
impl Eq for Decimal
```

## 常規用法及語義

Eq 是不含方法的標記 trait。相等關係比較數值，忽略小數位數及初始化狀態，並不意味著 JavaScript 物件身份相同。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-eq" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
