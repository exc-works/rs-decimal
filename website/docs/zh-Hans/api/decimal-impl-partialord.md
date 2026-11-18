---
locale: zh-Hans
source_revision: 25b1ec5a4bb0557629fac6d66ca48c015418489e6280f42443d66425e8372440
api_id: Decimal::impl PartialOrd
---
# Decimal::impl PartialOrd

通过 Rust 的偏序比较 Decimal。

```rust
impl PartialOrd for Decimal 
```

## 常规用法及语义

所有 Decimal 值都可比较，因此 partial_cmp 返回 Some(Ordering)。默认零可与构造的零比较，小数位数不同不会产生 None。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialord" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
