---
locale: zh-Hant
source_revision: aed80a094ad154f3abfbbdc72125d175f579082056ef3b224d8dda331e64cff5
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::into_rescaled
---
# Decimal::into_rescaled

消耗 Decimal，同時改變小數位數。

```rust
fn into_rescaled (self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

prec: isize；mode: RoundingMode。與 rescale 一樣，捨入或追加零，並始終初始化結果。即使 Rust 回傳 InvalidPrecision、RoundUnnecessary 或瀏覽器資源錯誤，接收者也已移動。JS 類型或範圍不合法會在進入 Rust 前被拒絕，不消耗接收者。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-rescaled" locale="zh-Hant" />

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
