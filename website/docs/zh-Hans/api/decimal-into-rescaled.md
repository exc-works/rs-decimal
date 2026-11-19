---
locale: zh-Hans
source_revision: aed80a094ad154f3abfbbdc72125d175f579082056ef3b224d8dda331e64cff5
api_id: Decimal::into_rescaled
---
# Decimal::into_rescaled

消耗 Decimal，同时改变小数位数。

```rust
fn into_rescaled (self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

prec: isize；mode: RoundingMode。与 rescale 一样，舍入或追加零，并始终初始化结果。即使 Rust 返回 InvalidPrecision、RoundUnnecessary 或浏览器资源错误，接收者也已移动。JS 类型或范围不合法会在进入 Rust 前被拒绝，不消耗接收者。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-rescaled" locale="zh-Hans" />

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
