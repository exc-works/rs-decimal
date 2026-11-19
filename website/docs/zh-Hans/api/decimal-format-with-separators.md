---
locale: zh-Hans
source_revision: c3fbe0927c540d91118929ccf7291b490ab739216719c7655f9f81744f8edff3
api_id: Decimal::format_with_separators
---
# Decimal::format_with_separators

对整数位分组，并选择小数分隔符。

```rust
fn format_with_separators (& self , thousands : char , decimal : char) -> String
```

## 参数、返回值及常规用法

thousands: char；decimal: char。返回 String/string，整数位每三位一组，保留所有存储的小数位。每个 JS 参数必须恰好是一个 Unicode 标量，可使用补充平面字符；孤立代理项和多字符字符串被拒绝。NUL thousands 禁用分组，NUL decimal 使用小数点。分隔符是字面字符，不是语言区域名称。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-with-separators" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
