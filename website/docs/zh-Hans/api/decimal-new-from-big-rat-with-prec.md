---
locale: zh-Hans
source_revision: 53d0bced8bfb282a55a5b5ecd0ca2b5866c74eb8bbf117b7537ea9c77cdf1b4d
api_id: Decimal::new_from_big_rat_with_prec
---
# Decimal::new_from_big_rat_with_prec

将有理数舍入到指定小数位数。

```rust
fn new_from_big_rat_with_prec (value : & BigRational , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

value: &BigRational；prec: 非负 isize；mode: RoundingMode。bridge 使用精确的 bigint 分子分母对象。与精确转换不同，允许 1/3 等无限小数。负精度返回 InvalidPrecision；Unnecessary 遇到必须舍入时返回 RoundUnnecessary。零分母在调用 Rust 前被 bridge 拒绝；零分子返回 prec 精度的已初始化零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat-with-prec" locale="zh-Hans" />

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
