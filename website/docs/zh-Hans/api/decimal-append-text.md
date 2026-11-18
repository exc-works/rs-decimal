---
locale: zh-Hans
source_revision: bf26b0f6b02acd5afbbeb54a43a96410bc32ae1ee6825c86870266f2a294720f
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

## 浏览器边界

遵循 [资源限制](../resource-policy)：格式宽度最多 65536 字节，precision 最多 1000，固定格式还检查系数增长。缓冲区总长最多 65536 字节，追加前使用保守长度估计，因此可能拒绝实际可容纳的临界操作。资源错误不会改变输入或缓冲区。print 对长文本另有显示上限；需要检查完整输出时可读取 length 或分段打印。

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
