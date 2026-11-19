---
locale: zh-Hans
source_revision: 44afbeff0d59cbde33e83e7f33c7e22c9903950f7ecf43d6f8230c6cde4b2ec4
api_id: "Decimal::marshal_bson_value"
---
# Decimal::marshal_bson_value

编码 BSON 类型字节和标量负载。

```rust
fn marshal_bson_value (& self) -> Result < (u8 , Vec < u8 >) , DecimalError >
```

## 参数、返回值及常规用法

无参数，返回 `Result<(u8, Vec<u8>), DecimalError>`/[number, Uint8Array]。Rust 需要 bson feature，浏览器构建已启用。元组包含 BSON 标量，而非完整 BSON 文档。已初始化值编码为 BSON 字符串（类型 2），不是 Decimal128；未初始化零编码为类型 10 和空负载。字符串负载先写小端 i32 字节长度（包含结尾 NUL），随后写无小数末尾零的规范文本和 NUL。返回字节是独立副本；编码不改变初始化或有效标志。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-bson-value" locale="zh-Hans" />

## 相关方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
