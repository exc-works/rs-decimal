---
locale: zh-Hans
source_revision: b225904d6e477d16ac1df6ba10bcfee3249227a343581851ba1cc46cc84f5863
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

## 浏览器边界

输入必须为 Uint8Array，kind 必须是 0–255 的整数。复制到 Wasm 前限制输入为 65536 字节；Rust 再检查二进制 scale 不超过 1000、实际系数不超过 16384 位。BSON 字符串需满足 8192 字节文本限制，Decimal128 的指数展开也接受 [资源检查](../resource-policy)。这些限制可能比原生库更严格。所有检查在接收者改变之前完成；Core 错误与资源错误保持区分，已移动的句柄不可复用。

## 相关方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
