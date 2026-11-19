---
locale: zh-Hans
source_revision: 5f1761508e55494581ac3ef80177b3b3480f53a919c2efbe231b66c02ccf3a14
api_id: GoFormat.precision
---
# GoFormat.precision

设置 precision 格式字段。

```rust
precision: Option < usize >
```

## 参数、返回值及常规用法

精度含义随动词变化：f/F 为小数位，e/E 为首位之后的位数，g/G 为有效数字（零按一处理），d 为最少整数位。None/null 使用动词默认规则；f 保留 scale，e 使用六位。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-precision" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
