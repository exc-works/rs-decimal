---
locale: zh-Hans
source_revision: f893b87d46c7c5082ca48cb1eae39734e63ca91a57f2f944773172a9baf78fca
api_id: GoFormat.zero
---
# GoFormat.zero

设置 zero 格式字段。

```rust
zero: bool
```

## 参数、返回值及常规用法

在符号之后用零填充数字左侧。left 为 true 或 d 有显式 precision 时忽略。默认为 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-zero" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
