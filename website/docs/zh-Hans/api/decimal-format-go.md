---
locale: zh-Hans
source_revision: f9b1eaa0c6a10e9f32908baa1f2c09a9aafeb3265bffa095bb0a0b210e1e1f05
api_id: Decimal::format_go
---
# Decimal::format_go

应用一个兼容 Go 的格式动词及选项。

```rust
fn format_go (& self , f : GoFormat) -> Result < String , DecimalError >
```

## 参数、返回值及常规用法

f: GoFormat；返回 Result<String, DecimalError>/string。接受单个动词，不接受 %.2f 这样的 printf 模板。支持 v/s/q/d/f/F/e/E/g/G/b。舍入使用 HalfEven。未知动词及对非整数使用 d 会产生 %! 文本标记，不抛出错误。定点精度超出原生有符号范围返回 InvalidPrecision；浏览器另有宽度、精度和分配限制。JS 省略的字段使用 Rust 默认值，未知字段和错误字段类型会被拒绝。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-go" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
