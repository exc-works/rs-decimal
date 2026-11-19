---
locale: zh-Hans
source_revision: 14996308295f3d9761be44a618304820607fea78ed2b85488ce4a81975821f38
api_id: Decimal::ln_with_prec
---
# Decimal::ln_with_prec

计算自然对数；Rust 返回 Result<Decimal, DecimalError>，bridge 返回 Decimal 或抛出结构化错误。prec: 非负 isize，以整数 number 表示，指定小数位数；负精度在定义域检查前返回 InvalidPrecision。输入必须为正；零和负数返回 InvalidLog。ln(1) 精确为零。核心使用区间包围，仅在舍入已验证时返回结果。使用 HalfEven，没有 mode 参数；核心预算内无法验证时返回 NoConvergence，不返回未经验证的近似值。输入保持不变，成功结果已初始化。

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
