---
locale: zh-Hans
source_revision: f90f9a719348f32512d576fc2ef23076c6492acd11b0dbe87ae7f8c72a7c2eab
api_id: "NullDecimal::marshal_json"
---
# NullDecimal::marshal_json

将 NullDecimal 编码为 json 字节。

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## 参数、返回值及常规用法

无参数，返回 `Vec<u8>`/Uint8Array，内容为JSON 字节。Rust 需要 json feature，浏览器构建已启用。无效 nullable 编码为 JSON null；有效值编码为带引号的规范小数字符串，即使内部 Decimal 未初始化。返回字节是独立副本，读取不改变状态。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-json" locale="zh-Hans" />

## 相关方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
