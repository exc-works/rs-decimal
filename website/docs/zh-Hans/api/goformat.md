---
locale: zh-Hans
source_revision: f89bca6a9b41c5dfaa6d534efc134c1f42c007ba065d09915785753b5a161115
api_id: GoFormat
---
# GoFormat

使用字段对象描述一次格式化操作。

```rust
pub struct GoFormat
```

## 参数、返回值及常规用法

字段对应 Rust 结构体：verb、width、precision、left、plus、space、zero、alternate。bridge 的 GoFormat.default() 从 Rust 读取默认值，返回新的可变对象；修改它不会设置全局格式。width/precision 使用 null 表示 None，非负 wasm32 整数 number 表示 Some。Rust trait 仍属于原生语法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
