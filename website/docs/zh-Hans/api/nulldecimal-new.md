---
locale: zh-Hans
source_revision: 6d83b155d9084cec9f42deea0ac618a5403b16dd0003e0789794c0428f3a019a
api_id: NullDecimal::new
---
# NullDecimal::new

把 Decimal 移入一个有效的可空值。

```rust
fn new (decimal : Decimal) -> Self
```

## 常规用法及语义

decimal: Decimal 会被消耗；结果始终 valid=true，即使 decimal 是未初始化的零。移动后的 JavaScript 输入不能继续使用。需要保留原值时应传入副本。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-new" locale="zh-Hans" />

这份共用示例同时在原生 Rust 和浏览器中执行。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
