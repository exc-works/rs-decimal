---
locale: zh-Hans
source_revision: a4afb74edf390a8e75f7a3a242d431661b5b997a3026d6a0446a646f7a0d41bb
api_id: Decimal::exp_with_prec
---
# Decimal::exp_with_prec

计算指数 e^x。prec: 非负 isize，以整数 number 表示，指定小数位数；负精度在定义域检查前返回 InvalidPrecision。核心接受所有实数输入。exp(0) 是已初始化的 1；足够负的输入可在指定精度下正确舍入为已初始化的零。核心使用区间包围，仅在舍入已验证时返回结果。使用 HalfEven，没有 mode 参数；核心预算内无法验证时返回 NoConvergence，不返回未经验证的近似值。输入保持不变，成功结果已初始化。

```rust
fn exp_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 常规示例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-exp-with-prec" locale="zh-Hans" />

## 相关方法

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
