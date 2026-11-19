---
locale: zh-Hans
source_revision: c5619d120e2ab677bb25ae58cef9205a1160d82cd87b1081f2a6532750a83303
api_id: GoFormat.space
---
# GoFormat.space

设置 space 格式字段。

```rust
space: bool
```

## 参数、返回值及常规用法

plus 为 false 时，为非负数字输出添加前导空格。负数保留符号。默认为 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-space" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
