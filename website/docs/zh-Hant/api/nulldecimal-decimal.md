---
locale: zh-Hant
source_revision: 74688d1ee73b78a25c1792ba73a6a2e013084b0279376cef739a1b5c8cbed64f
translation_provenance: zh-Hans-terminology-reviewed
api_id: NullDecimal.decimal
---
# NullDecimal.decimal

獨立於有效性旗標查看或替換內部 Decimal。

```rust
decimal: Decimal
```

## 常規用法及語義

Rust 暴露可變的 Decimal 欄位。瀏覽器使用 get_decimal_clone() 取得獨立副本，透過 set_decimal(value) 消耗並替換新值。兩種適配器都不改變 valid，避免在 JavaScript 中承諾即時的 Rust 參照。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-decimal" locale="zh-Hant" />

這份共用範例同時在原生 Rust 和瀏覽器中執行。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
