---
locale: zh-Hant
source_revision: 9743ba1e89051026e818a070b86183de419a65ca646afc51459afa17731968b9
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::int_part
---
# Decimal::int_part

向零截斷，提取整數部分。

```rust
fn int_part (& self) -> BigInt
```

## 參數、回傳值及常規用法

無參數，Rust 回傳 BigInt，bridge 回傳 bigint，不經過浮點轉換。-12.75 得到 -12，而非 floor(-12.75) 的 -13。預設零得到零。範例將整數序列化為字串，以在 JSON 中保留任意精度。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="zh-Hant" />

## 瀏覽器邊界

輸入均被借用，成功和普通錯誤都不會消耗輸入。回傳的 Decimal 獨立擁有資料，算術結果已初始化。瀏覽器在呼叫核心前檢查 [資源限制](../resource-policy)：對齊、乘積中間係數、除法縮放及輸出精度。保守的預檢查可能拒絕最終結果很小的運算。資源錯誤與數值程式庫錯誤分開報告；panic 會終止執行。

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
