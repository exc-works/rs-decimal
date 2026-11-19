---
locale: zh-Hans
source_revision: 8a589312845f2d23d85f3d6da1987ee87f79dfecdc97a705386e9a336b17d7ed
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

## 相关方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
