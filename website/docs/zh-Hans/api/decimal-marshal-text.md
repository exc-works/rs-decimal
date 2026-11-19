---
locale: zh-Hans
source_revision: dabe3e417d155ec62acc296ebb36a3f87bdc044baf1a1ffd4eb27cbaf056b77d
api_id: "Decimal::marshal_text"
---
# Decimal::marshal_text

将 Decimal 编码为 text 字节。

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## 参数、返回值及常规用法

无参数，返回 `Vec<u8>`/Uint8Array，内容为规范的十进制 ASCII 文本。移除小数末尾零。默认 Decimal 零输出 ASCII 0。返回字节是独立副本，读取不改变状态。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-text" locale="zh-Hans" />

## 相关方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
