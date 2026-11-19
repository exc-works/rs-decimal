---
locale: zh-Hans
source_revision: fbc64b05e8340067582332c66c6f66973b00c5a17412c42b00f322b92b77d915
api_id: Decimal::ln_with_prec
---
# Decimal::ln_with_prec

计算自然对数。prec: 非负 isize，以整数 number 表示，指定小数位数；负精度在定义域检查前返回 InvalidPrecision。输入必须为正；零和负数返回 InvalidLog。ln(1) 精确为零。核心使用区间包围，仅在舍入已验证时返回结果。使用 HalfEven，没有 mode 参数；核心预算内无法验证时返回 NoConvergence，不返回未经验证的近似值。输入保持不变，成功结果已初始化。

```rust
fn ln_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 常规示例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-ln-with-prec" locale="zh-Hans" />

## 相关方法

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
