---
locale: zh-Hans
source_revision: 8de700fe2bea77817d4cc64ce868f77fd5301e630577d698cc1cf5a3b4f76ce5
api_id: "reexport::PRECISION_FIXED_SIZE"
---
# reexport::PRECISION_FIXED_SIZE

读取二进制精度前缀的字节数。

```rust
pub use binary::PRECISION_FIXED_SIZE as PRECISION_FIXED_SIZE
```

## 参数、返回值及常规用法

Rust usize 常量，值为 4：兼容 Go 的二进制格式中大端 scale 前缀的字节数。它不是小数位数或精度上限。浏览器提供来自 Rust 的只读 number。crate 根重导出属于 Rust 导入语法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-reexport-precision-fixed-size" locale="zh-Hans" />

## 相关方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
