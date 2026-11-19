---
locale: zh-Hans
source_revision: 6440a61a644ecdbf15d2da0254f2f46ec6a0bac106e6a427225c0f45a05d250f
api_id: Decimal::must_non_negative
---
# Decimal::must_non_negative

要求数值非负，返回已初始化的副本。

```rust
fn must_non_negative (& self) -> Self
```

## 参数、返回值及常规用法

无参数；非负输入（包括零）返回 Decimal。负输入在 Rust 中以字符串载荷 panic；在 Wasm 中是 trap，不是可恢复的 DecimalError，会结束本次 playground 运行。重新运行会创建新的 Wasm 状态。这是数值断言，不是绝对值转换；改变符号请使用 abs。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-must-non-negative" locale="zh-Hans" />

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
