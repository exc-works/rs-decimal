---
locale: zh-Hans
source_revision: b07552403b149d5a66b24c1aa4c0547e8259f4d124ac401ca1161c70929a6aa6
api_id: "NullDecimal::marshal_bson_value"
---
# NullDecimal::marshal_bson_value

编码 BSON 类型字节和标量负载。

```rust
fn marshal_bson_value (& self) -> Result < (u8 , Vec < u8 >) , DecimalError >
```

## 参数、返回值及常规用法

无参数，返回 `Result<(u8, Vec<u8>), DecimalError>`/[number, Uint8Array]。Rust 需要 bson feature，浏览器构建已启用。元组包含 BSON 标量，而非完整 BSON 文档。有效 nullable 值编码为 BSON 字符串，包括内部零未初始化的有效值；无效值编码为类型 10 和空负载。字符串负载先写小端 i32 字节长度（包含结尾 NUL），随后写无小数末尾零的规范文本和 NUL。返回字节是独立副本；编码不改变初始化或有效标志。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-nulldecimal-marshal-bson-value" locale="zh-Hans" />

## 浏览器边界

输入必须为 Uint8Array，kind 必须是 0–255 的整数。复制到 Wasm 前限制输入为 65536 字节；Rust 再检查二进制 scale 不超过 1000、实际系数不超过 16384 位。BSON 字符串需满足 8192 字节文本限制，Decimal128 的指数展开也接受 [资源检查](../resource-policy)。这些限制可能比原生库更严格。所有检查在接收者改变之前完成；Core 错误与资源错误保持区分，已移动的句柄不可复用。

## 相关方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
