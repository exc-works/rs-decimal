---
locale: zh-Hans
source_revision: 0989f60fe84cf7be4e7708a7140f9cc16468c21ad57d855b67f896084ddd8606
api_id: GoFormat.plus
---
# GoFormat.plus

设置 plus 格式字段。

```rust
plus: bool
```

## 参数、返回值及常规用法

对非负数字输出显示 +，优先于 space。适用于数字动词，不适用于 v/s/q；负数保留 -。默认为 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-plus" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
