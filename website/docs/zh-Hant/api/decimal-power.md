---
locale: zh-Hant
source_revision: 6c8f91b0409cc71714a4b9926acf34a3cf0aa635f8904f2e607ed3243fb8b30a
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::power
---
# Decimal::power

計算整數次冪，保留原始小數位數。

```rust
fn power (& self , power : i64) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

power: i64，以 bigint 傳輸。按 HalfEven 捨入到輸入小數位數，負指數取倒數。2.00 的 -3 次冪得到 0.12，而整數精度的 2 的 -3 次冪得到 0。零的零次冪是已初始化的 1；零的負次冪回傳 DivideByZero。超過核心中間值上限的巨大指數在分配前回傳 Overflow。瀏覽器還有更小的中間精度和係數限制；0 和正負 1 保留核心快速路徑。輸入僅被借用。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-power" locale="zh-Hant" />

## 瀏覽器邊界

遵循上述所有權規則。移動後的物件再次存取會回傳 moved 錯誤；free 可重複呼叫。類型或別名檢查在移動前執行，進入 Rust 後的錯誤不會恢復已移動的接收者。[資源限制](../resource-policy) 獨立於程式庫錯誤；冪運算檢查中間值，對齊運算檢查係數增長。保守檢查可能拒絕最終結果很小的運算。

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
