---
locale: zh-Hans
source_revision: 65435d6f54d8777702cf2f629258b85e312b71241d8a0725f7c209d6e7b4a200
api_id: "PRECISION_FIXED_SIZE"
---
# PRECISION_FIXED_SIZE

读取二进制精度前缀的字节数。

```rust
const PRECISION_FIXED_SIZE: usize = 4
```

## 参数、返回值及常规用法

Rust usize 常量，值为 4：兼容 Go 的二进制格式中大端 scale 前缀的字节数。它不是小数位数或精度上限。浏览器提供来自 Rust 的只读 number。crate 根重导出属于 Rust 导入语法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-precision-fixed-size" locale="zh-Hans" />

## 相关方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
