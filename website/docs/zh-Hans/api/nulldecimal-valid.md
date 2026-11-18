---
locale: zh-Hans
source_revision: d57c1e8ddb07024ed3c5c002f095b2cee086c1a0719df70a0b4ba8761f29418d
api_id: NullDecimal.valid
---
# NullDecimal.valid

读取或修改是否存在值，不改变内部 Decimal。

```rust
valid: bool
```

## 常规用法及语义

Rust 公开的 bool 字段映射为 boolean getter/setter。设置 false 不会清除 decimal，设置 true 也不会将其初始化。bridge 会在修改前拒绝非 boolean 输入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-valid" locale="zh-Hans" />

这份共用示例同时在原生 Rust 和浏览器中执行。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
