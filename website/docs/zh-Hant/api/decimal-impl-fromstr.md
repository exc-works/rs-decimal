---
locale: zh-Hant
source_revision: 65703feb0e11af9dfe348c058c514dbf10ab13c1d2a720632926e57fee628fa9
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::impl FromStr
---
# Decimal::impl FromStr

透過 Rust 的 parse 和 FromStr 解析小數文字。

```rust
impl FromStr for Decimal
```

## 常規用法及語義

回傳 Decimal 或 DecimalError。語法及零值標準化與 new_from_string 相同。瀏覽器呼叫該具名方法；Rust trait 語法僅適用於原生 Rust。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
