---
locale: zh-Hans
source_revision: 0c3a3cc5d3969cdd2cb971de3686daa945e18486f55f8c99179deb863727c501
api_id: GoFormat.verb
---
# GoFormat.verb

设置 verb 格式字段。

```rust
verb: char
```

## 参数、返回值及常规用法

选择单个 Unicode 标量格式动词。默认 v 使用紧凑定点文本；e/E 使用科学记数法，q 加引号，b 输出二进制系数及 p-scale（不是 IEEE 浮点格式）。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-verb" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
