---
locale: zh-Hant
source_revision: 317e1f4c19916312bb2823fc0a526f5311afada3dc44c8d3247bdd5fc0f08111
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::impl Ord::cmp
---
# Decimal::impl Ord::cmp

透過 Rust Ord 按數值對 Decimal 排序。

```rust
fn cmp (& self , other : & Self) -> Ordering
```

## 常規用法及語義

Ord::cmp 回傳 Ordering，不同於回傳 -1/0/1 的 Decimal::cmp。表示差異不影響排序。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-ord-cmp" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
