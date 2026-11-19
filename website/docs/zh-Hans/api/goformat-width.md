---
locale: zh-Hans
source_revision: fe33f9fd8631eeeae5a499a47abbc262340fc190757785abe0749a14c97bf791
api_id: GoFormat.width
---
# GoFormat.width

设置 width 格式字段。

```rust
width: Option < usize >
```

## 参数、返回值及常规用法

最小宽度包含符号和小数点；通过填充补足，不截断更长文本。None/null 禁用最小宽度。核心按输出字节数计算。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-width" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
