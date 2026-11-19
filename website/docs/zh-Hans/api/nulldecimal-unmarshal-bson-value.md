---
locale: zh-Hans
source_revision: 59bfa6e69fb4720796373509ad0a7f4325fcfd6dbb9c49ba19604b5d730cd4a1
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

## 相关方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
