---
locale: zh-Hans
source_revision: 01c476851989c18cecc0970b361010eee7cb4cfd902d7e4240be18102abc0899
api_id: Decimal::power
---
# Decimal::power

计算整数次幂，保留原始小数位数。

```rust
fn power (& self , power : i64) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

power: i64，以 bigint 传输。按 HalfEven 舍入到输入小数位数，负指数取倒数。2.00 的 -3 次幂得到 0.12，而整数精度的 2 的 -3 次幂得到 0。零的零次幂是已初始化的 1；零的负次幂返回 DivideByZero。超过核心中间值上限的巨大指数在分配前返回 Overflow。浏览器还有更小的中间精度和系数限制；0 和正负 1 保留核心快速路径。输入仅被借用。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-power" locale="zh-Hans" />

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
