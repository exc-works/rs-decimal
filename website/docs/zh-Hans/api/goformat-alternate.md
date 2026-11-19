---
locale: zh-Hans
source_revision: 711eb90b21b1b79f0303fceb3e98f79a6b9b78a5eb632f30a661f662027c1167
api_id: GoFormat.alternate
---
# GoFormat.alternate

设置 alternate 格式字段。

```rust
alternate: bool
```

## 参数、返回值及常规用法

对 f/F/e/E 保留小数点，对 g/G 保留一般格式的末尾零。不是改变数字进制。默认为 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-alternate" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
