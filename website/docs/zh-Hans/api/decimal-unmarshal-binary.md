---
locale: zh-Hans
source_revision: 404c61af98ea51d383b516ba1e691c0c718c44923614dbeaca2d2a7efd410ffc
api_id: "Decimal::unmarshal_binary"
---
# Decimal::unmarshal_binary

从兼容 Go 的二进制标量替换小数。

```rust
fn unmarshal_binary (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## 参数、返回值及常规用法

bytes: &[u8]/Uint8Array；修改接收者，返回 `Result<(), DecimalError>`/undefined。前四字节是大端 scale，之后为 Gob 头 2（正）或 3（负）和绝对值。成功解码保留编码中的 scale。空输入产生已初始化零，与默认未初始化零不同。只有四字节前缀而无 Gob 负载时也接受，产生该 scale 的已初始化零。过短前缀、无效版本或超过 131072 的 scale 返回核心 Unmarshal 错误。错误保留接收者。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-binary" locale="zh-Hans" />

## 浏览器边界

输入必须为 Uint8Array，kind 必须是 0–255 的整数。复制到 Wasm 前限制输入为 65536 字节；Rust 再检查二进制 scale 不超过 1000、实际系数不超过 16384 位。BSON 字符串需满足 8192 字节文本限制，Decimal128 的指数展开也接受 [资源检查](../resource-policy)。这些限制可能比原生库更严格。所有检查在接收者改变之前完成；Core 错误与资源错误保持区分，已移动的句柄不可复用。

## 相关方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
