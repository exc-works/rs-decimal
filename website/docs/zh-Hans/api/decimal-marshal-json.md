---
locale: zh-Hans
source_revision: 76052d80c386b89ac2428898d57b9ce525f22161fd5cd3ee7956cceafe3b11e9
api_id: "Decimal::marshal_json"
---
# Decimal::marshal_json

将 Decimal 编码为 json 字节。

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## 参数、返回值及常规用法

无参数，返回 `Vec<u8>`/Uint8Array，内容为JSON 字节。Rust 需要 json feature，浏览器构建已启用。已初始化的 Decimal 编码为带引号的规范字符串；默认未初始化零编码为 JSON null。返回字节是独立副本，读取不改变状态。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-json" locale="zh-Hans" />

## 相关方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
