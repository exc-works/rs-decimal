---
locale: zh-Hans
source_revision: 0268b5969d6d0344c17f1c1c3dee925ce05b68c8ea0fc3782a3dde3b820e544c
api_id: "config::decode_decimal"
---
# config::decode_decimal

解码明确类型的配置值。

```rust
fn decode_decimal (value : ConfigValue < '_ >) -> Result < Decimal , DecimalError >
```

## 参数、返回值及常规用法

value: ConfigValue；返回 `Result<Decimal, DecimalError>`/新的 Decimal 句柄。Rust 需要 config feature。不可空解码拒绝 Null 和无效 Nullable，不自动补零。现有 Decimal 和有效 Nullable 按完整表示克隆，包括未初始化零。Text/Number/Bytes 精确解析小数文本，不移除 SQL 引号。浮点数已带二进制舍入；保留原始数字时用文本或 bigint。浏览器变体属于本次运行，借用句柄或缓冲区至解码调用。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-decimal" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
