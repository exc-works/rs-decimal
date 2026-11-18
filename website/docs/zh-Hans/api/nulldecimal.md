---
locale: zh-Hans
source_revision: 8675e68949343d33893aa6f81cc6a57f32cdafd3aaf80161520178113f46a31d
api_id: NullDecimal
---
# NullDecimal

用独立的有效标记和存储值表示可空 Decimal。

```rust
pub struct NullDecimal
```

## 常规用法及语义

是否存在值仅由 valid 决定。无效的可空值可以保留非零 decimal；有效的可空值也可以包含未初始化的零。Rust 的公开 decimal 字段在 JavaScript 中通过明确的复制及移动适配器访问。默认值表示缺失。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal" locale="zh-Hans" />

这份共用示例同时在原生 Rust 和浏览器中执行。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
