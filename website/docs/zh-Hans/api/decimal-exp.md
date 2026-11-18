---
locale: zh-Hans
source_revision: 4812fffbe74c100e5632736fe3ca1c5dd726efa9358bb2ca70cc5db51e55bd14
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

## 浏览器边界

这组方法有额外的 [资源限制](../resource-policy)：输出精度最多 128，输入 scale 最多 256，输入系数最多 4096 个二进制位。exp 的普通计算还要求绝对值不超过 64；核心可直接证明舍入为零的负输入保留快速路径。较大的输入 scale 可能使无参数方法的默认精度超过 128，可用 with_prec 明确选择允许的输出精度。资源错误与库的 NoConvergence 不同。运行可通过 Stop 中断；重新运行使用新的 Wasm 实例。

## 相关方法

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
