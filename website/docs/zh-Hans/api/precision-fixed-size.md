---
locale: zh-Hans
source_revision: 73595bceae9a4435e67075be3cf3874619847ef2c8e049d0faeae85eae4feb26
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

## 浏览器边界

输入必须为 Uint8Array，kind 必须是 0–255 的整数。复制到 Wasm 前限制输入为 65536 字节；Rust 再检查二进制 scale 不超过 1000、实际系数不超过 16384 位。BSON 字符串需满足 8192 字节文本限制，Decimal128 的指数展开也接受 [资源检查](../resource-policy)。这些限制可能比原生库更严格。所有检查在接收者改变之前完成；Core 错误与资源错误保持区分，已移动的句柄不可复用。

## 相关方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
