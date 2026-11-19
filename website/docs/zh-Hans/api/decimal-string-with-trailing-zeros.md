---
locale: zh-Hans
source_revision: f320c1a30df05777794b5d83edd6494fed5ebfe3f6b80e1d48ebc83fa5e17748
api_id: Decimal::string_with_trailing_zeros
---
# Decimal::string_with_trailing_zeros

格式化为定点十进制文本。

```rust
fn string_with_trailing_zeros (& self) -> String
```

## 参数、返回值及常规用法

无参数，返回 String/string，不使用科学记数法。保留所有存储的小数位，包括末尾零。使用小数点，不分组。负零没有单独的表示。默认零输出 0，不因此被初始化；格式化仅借用输入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string-with-trailing-zeros" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
