---
locale: zh-Hans
source_revision: 9b1c2ac4982ef348ecf9982f858e7f552532e21dc0129fa816673741276acaf7
api_id: GoFormat.left
---
# GoFormat.left

设置 left 格式字段。

```rust
left: bool
```

## 参数、返回值及常规用法

在右侧填充空格。左对齐优先于数字零填充。默认为 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-left" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
