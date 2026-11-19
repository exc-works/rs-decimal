---
locale: zh-Hant
source_revision: 6440a61a644ecdbf15d2da0254f2f46ec6a0bac106e6a427225c0f45a05d250f
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::must_non_negative
---
# Decimal::must_non_negative

要求數值非負，回傳已初始化的副本。

```rust
fn must_non_negative (& self) -> Self
```

## 參數、回傳值及常規用法

無參數；非負輸入（包括零）回傳 Decimal。負輸入在 Rust 中以字串載荷 panic；在 Wasm 中是 trap，不是可恢復的 DecimalError，會結束本次 playground 執行。重新執行會建立新的 Wasm 狀態。這是數值斷言，不是絕對值轉換；改變符號請使用 abs。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-must-non-negative" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
