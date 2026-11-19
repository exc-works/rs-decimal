---
locale: zh-Hans
source_revision: 2a36a5c46f57052b68644728359c44f15465f6bfca2c60d1540e40c603cc8c5b
api_id: NullDecimal.valid
---
# NullDecimal.valid

读取或修改是否存在值，不改变内部 Decimal。

```rust
valid: bool
```

## 常规用法及语义

Rust 公开的 bool 字段映射为 boolean getter/setter。设置 false 不会清除 decimal，设置 true 也不会将其初始化。JavaScript 会在修改前拒绝非 boolean 输入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-valid" locale="zh-Hans" />

这份共用示例同时在原生 Rust 和浏览器中执行。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
