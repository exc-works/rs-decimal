---
locale: zh-Hant
source_revision: d4492d82b410cc5d7b67274496227b1fa1d6279b6d276d6bd8bfbe25ef422dd4
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::remainder
---
# Decimal::remainder

拆分整數部分和未縮放的小數餘數。

```rust
fn remainder (& self) -> (BigInt , BigInt)
```

## 參數、回傳值及常規用法

無參數；回傳 (BigInt, BigInt)，映射為 [bigint, bigint]。第一項向零截斷；第二項是帶符號的係數餘數，不是 Decimal 小數。-12.750 得到 (-12, -750)：第二項需要原始 scale 3 才表示 -0.750。預設零回傳 (0, 0)。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-remainder" locale="zh-Hant" />

## 瀏覽器邊界

遵循上述所有權規則。移動後的物件再次存取會回傳 moved 錯誤；free 可重複呼叫。類型或別名檢查在移動前執行，進入 Rust 後的錯誤不會恢復已移動的接收者。[資源限制](../resource-policy) 獨立於程式庫錯誤；冪運算檢查中間值，對齊運算檢查係數增長。保守檢查可能拒絕最終結果很小的運算。

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
