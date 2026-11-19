---
locale: zh-Hans
source_revision: 19e6388558d46ddb3598af453ea4bcd2872fc2f6ce5d4c459e0dfbab37170bfa
api_id: Decimal::log2
---
# Decimal::log2

计算以 2 为底的对数。无参数。默认输出精度为 max(输入小数位数, 20)，不是 20 位有效数字。输入必须为正；零和负数返回 InvalidLog。底数的精确幂具有整数对数，并保留输出小数位数。核心使用区间包围，仅在舍入已验证时返回结果。使用 HalfEven，没有 mode 参数；核心预算内无法验证时返回 NoConvergence，不返回未经验证的近似值。输入保持不变，成功结果已初始化。

```rust
fn log2 (& self) -> Result < Self , DecimalError >
```

## 常规示例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log2" locale="zh-Hans" />

## 相关方法

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
