---
locale: zh-Hans
source_revision: cc1566925b96baa324fb64a51eba19d80177bb315f68b0f6a9ee1d05c6da9807
api_id: Decimal::exp
---
# Decimal::exp

计算指数 e^x；Rust 返回 Result<Decimal, DecimalError>，bridge 返回 Decimal 或抛出结构化错误。无参数。默认输出精度为 max(输入小数位数, 20)，不是 20 位有效数字。核心接受所有实数输入。exp(0) 是已初始化的 1；足够负的输入可在指定精度下正确舍入为已初始化的零。核心使用区间包围，仅在舍入已验证时返回结果。使用 HalfEven，没有 mode 参数；核心预算内无法验证时返回 NoConvergence，不返回未经验证的近似值。输入保持不变，成功结果已初始化。

```rust
fn exp (& self) -> Result < Self , DecimalError >
```

## 常规示例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-exp" locale="zh-Hans" />

## 相关方法

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
