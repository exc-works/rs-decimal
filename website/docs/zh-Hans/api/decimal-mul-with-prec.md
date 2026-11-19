---
locale: zh-Hans
source_revision: b2363400afff1e4c740938a16816a148faaff0faef10b304c45a776ef2dcc1ee
api_id: Decimal::mul_with_prec
---
# Decimal::mul_with_prec

按明确的输出精度相乘。

```rust
fn mul_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

other: &Decimal。结果保留 prec 个小数位；prec 必须非负，否则返回 InvalidPrecision。显式 mode 控制舍入；Unnecessary 在必须丢弃非零数字时返回 RoundUnnecessary。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-with-prec" locale="zh-Hans" />

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
