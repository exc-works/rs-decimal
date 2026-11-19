---
locale: zh-Hans
source_revision: dd4986a8dd71923263f954c74b5685b1dfaa3a4b594ef1aadb36c8a62519bd62
api_id: Decimal::append_text
---
# Decimal::append_text

向现有可变缓冲区追加十进制 ASCII 字节。

```rust
fn append_text (& self , dst : & mut Vec < u8 >)
```

## 参数、返回值及常规用法

`dst: &mut Vec<u8>`，以 ByteBuffer 对象表示。Rust 返回 ()，JS 返回 undefined。保留已有字节，原地修改目标。移除小数末尾零。Decimal 和缓冲区都仍可使用。ByteBuffer.new 复制 Uint8Array；to_uint8_array 返回副本，不是 Wasm 内存视图。资源错误发生在修改前，保留原缓冲区。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-append-text" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
