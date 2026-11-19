---
locale: zh-Hans
source_revision: 2b341a7995c77dbb72c2087390bcd56b4208d2c9ec45de8ea22c2ce3f209501a
api_id: "NullDecimal::marshal_text"
---
# NullDecimal::marshal_text

将 NullDecimal 编码为 text 字节。

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## 参数、返回值及常规用法

无参数，返回 `Vec<u8>`/Uint8Array，内容为规范的十进制 ASCII 文本。有效值编码为规范小数文本；无效值编码为空字节，不是单词 null。返回字节是独立副本，读取不改变状态。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-text" locale="zh-Hans" />

## 相关方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
