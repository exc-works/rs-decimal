---
locale: zh-Hant
source_revision: 9c252d1ff038e1dac4451e4dd728020f00ea8f88567265b756b2f63f4474b26e
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

## 瀏覽器邊界

輸入均被借用，成功和普通錯誤都不會消耗輸入。回傳的 Decimal 獨立擁有資料，算術結果已初始化。瀏覽器在呼叫核心前檢查 [資源限制](../resource-policy)：對齊、乘積中間係數、除法縮放及輸出精度。保守的預檢查可能拒絕最終結果很小的運算。資源錯誤與數值程式庫錯誤分開報告；panic 會終止執行。

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
