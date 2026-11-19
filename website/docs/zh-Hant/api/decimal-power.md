---
locale: zh-Hant
source_revision: 01c476851989c18cecc0970b361010eee7cb4cfd902d7e4240be18102abc0899
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

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
