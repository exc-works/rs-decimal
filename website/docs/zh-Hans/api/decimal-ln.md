---
locale: zh-Hans
source_revision: 1b4d2150673750e48078239ad7618480e7fc5bb95b241e76137fd9b50e2c070d
api_id: Decimal::ln
---
# Decimal::ln

计算自然对数。无参数。默认输出精度为 max(输入小数位数, 20)，不是 20 位有效数字。输入必须为正；零和负数返回 InvalidLog。ln(1) 精确为零。核心使用区间包围，仅在舍入已验证时返回结果。使用 HalfEven，没有 mode 参数；核心预算内无法验证时返回 NoConvergence，不返回未经验证的近似值。输入保持不变，成功结果已初始化。

```rust
fn ln (& self) -> Result < Self , DecimalError >
```

## 常规示例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-ln" locale="zh-Hans" />

## 相关方法

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
