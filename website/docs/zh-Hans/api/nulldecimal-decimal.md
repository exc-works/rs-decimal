---
locale: zh-Hans
source_revision: 74688d1ee73b78a25c1792ba73a6a2e013084b0279376cef739a1b5c8cbed64f
api_id: NullDecimal.decimal
---
# NullDecimal.decimal

独立于有效标记查看或替换内部 Decimal。

```rust
decimal: Decimal
```

## 常规用法及语义

Rust 暴露可变的 Decimal 字段。浏览器使用 get_decimal_clone() 获取独立副本，通过 set_decimal(value) 消耗并替换新值。两种适配器都不改变 valid，避免在 JavaScript 中承诺实时的 Rust 引用。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-decimal" locale="zh-Hans" />

这份共用示例同时在原生 Rust 和浏览器中执行。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
