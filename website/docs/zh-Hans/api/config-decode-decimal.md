---
locale: zh-Hans
source_revision: 9abf8678e26851a7c2e2283fba52cbf7f426bb86feeae64e6b8312a6a7254ba7
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

## 浏览器边界

config 与 serde feature 已启用。源文本和字节最多 8192 个 UTF-8 字节；Rust 在解析前检查小数文本和 JSON 解码后的字符串，因此转义不能绕过 [资源限制](../resource-policy)。128 位整数必须使用范围内的 bigint。借用的 Decimal/NullDecimal 在解码时必须仍有效，转换生成独立值，不消耗输入。Serde 的通用 trait 语法由具体 JSON 适配器演示。

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
