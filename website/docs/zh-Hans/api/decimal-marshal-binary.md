---
locale: zh-Hans
source_revision: aa67b8e6e6f3fb7b6a66f11573d9907fb9dc057abd9547e4d0c0474969528084
api_id: "Decimal::marshal_binary"
---
# Decimal::marshal_binary

编码兼容 Go 的 precision/Gob 二进制格式。

```rust
fn marshal_binary (& self) -> Result < Vec < u8 > , DecimalError >
```

## 参数、返回值及常规用法

无参数，返回 `Result<Vec<u8>, DecimalError>`/Uint8Array。已初始化值先移除小数末尾零，再写入四字节大端 scale、big.Int Gob 版本/符号字节和大端绝对值。这是小数标量格式，不是 IEEE 浮点数。未初始化零编码为空字节，已初始化零编码为 [0,0,0,0,2]。返回的是独立副本，修改字节不改变 Decimal。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-binary" locale="zh-Hans" />

## 相关方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
