---
locale: zh-Hans
source_revision: a350b5bea887bf82e5e3ddafba6453f683a9f5862d842b3e641b0a441cbb3289
api_id: "NullDecimal::unmarshal_bson_value"
---
# NullDecimal::unmarshal_bson_value

将 BSON 标量解码到现有值。

```rust
fn unmarshal_bson_value (& mut self , kind : u8 , data : & [u8]) -> Result < () , DecimalError >
```

## 参数、返回值及常规用法

kind: u8/number，data: &[u8]/Uint8Array；修改接收者，返回 `Result<(), DecimalError>`/undefined。Rust 需要 bson。支持 double（1）、string（2）、null（10）、int32（16）、int64（18）和 Decimal128（19）；数值负载使用 BSON 小端字节。int64 应直接传原始字节，避免 JS number 丢失精度。BSON null 清除 valid 并将内部 Decimal 重置为未初始化零。nullable 成功解码非空值后设置 valid=true。不支持的类型、截断数据和非有限数值均失败且不改变状态。兼容细节：字符串解码跳过声明的最后一个字节而不验证 NUL；超出所需负载的字节被忽略。Decimal128 由 Rust 核心解码并保留支持的小数精度；marshal_bson_value 输出仍是字符串。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-nulldecimal-unmarshal-bson-value" locale="zh-Hans" />

## 浏览器边界

输入必须为 Uint8Array，kind 必须是 0–255 的整数。复制到 Wasm 前限制输入为 65536 字节；Rust 再检查二进制 scale 不超过 1000、实际系数不超过 16384 位。BSON 字符串需满足 8192 字节文本限制，Decimal128 的指数展开也接受 [资源检查](../resource-policy)。这些限制可能比原生库更严格。所有检查在接收者改变之前完成；Core 错误与资源错误保持区分，已移动的句柄不可复用。

## 相关方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
