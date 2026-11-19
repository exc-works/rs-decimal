---
locale: zh-Hans
source_revision: 03da42d48c5d8b250daaaa93d59162466d4ca8b88da11e32c7b577b2d36d0747
api_id: "config"
---
# config

解码明确类型的配置值。

```rust
pub mod config
```

## 参数、返回值及常规用法

此配置适配器支持文本/文本字节、精确数字文本、有符号或无符号 128 位整数、浮点数以及借用的 Decimal/NullDecimal。Rust 需要 config feature。不可空解码拒绝 Null 和无效 Nullable，不自动补零。现有 Decimal 和有效 Nullable 按完整表示克隆，包括未初始化零。Text/Number/Bytes 精确解析小数文本，不移除 SQL 引号。浮点数已带二进制舍入；保留原始数字时用文本或 bigint。浏览器变体属于本次运行，借用句柄或缓冲区至解码调用。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
