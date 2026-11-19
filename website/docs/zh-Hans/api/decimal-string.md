---
locale: zh-Hans
source_revision: 25693aaef850a3e88dcc638a63de0aa32a65fe1057c9bac969c2400862072704
api_id: Decimal::string
---
# Decimal::string

格式化为定点十进制文本。

```rust
fn string (& self) -> String
```

## 参数、返回值及常规用法

无参数，返回 String/string，不使用科学记数法。移除多余的小数末尾零，保留整数末尾零。使用小数点，不分组。负零没有单独的表示。默认零输出 0，不因此被初始化；格式化仅借用输入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
