---
locale: zh-Hans
source_revision: 38cd98cd0eb4f9b158b56009f272819540665b187722c4eef35585af26929132
api_id: COMPATIBILITY_REVISION
---
# COMPATIBILITY_REVISION

查看兼容性检查对应的上游 Go 版本。

```rust
const COMPATIBILITY_REVISION: & str = "a8791208e2d15483cf652cd2814b74b0c1a84187"
```

## 常规用法及语义

这个不可变的 &str 是源码版本，不是当前 Rust 或网页构建提交。浏览器从编译后的 Rust 读取同一常量，不应将它当作包版本或部署标识。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-compatibility-revision" locale="zh-Hans" />

这份共用示例同时在原生 Rust 和浏览器中执行。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
