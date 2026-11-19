---
locale: zh-Hans
source_revision: 50a6a7b47a0234b339cf6d517b9928ec7d1775457e7eacbe40da86286996212a
api_id: "Decimal::unmarshal_bson_value"
---
# Decimal::unmarshal_bson_value

将 BSON 标量解码到现有值。

```rust
fn unmarshal_bson_value (& mut self , kind : u8 , data : & [u8]) -> Result < () , DecimalError >
```

## 参数、返回值及常规用法

kind: u8/number，data: &[u8]/Uint8Array；修改接收者，返回 `Result<(), DecimalError>`/undefined。Rust 需要 bson。支持 double（1）、string（2）、null（10）、int32（16）、int64（18）和 Decimal128（19）；数值负载使用 BSON 小端字节。int64 应直接传原始字节，避免 JS number 丢失精度。BSON null 将接收者重置为未初始化零，这与保留原值的 unmarshal_json(null) 不同。nullable 成功解码非空值后设置 valid=true。不支持的类型、截断数据和非有限数值均失败且不改变状态。兼容细节：字符串解码跳过声明的最后一个字节而不验证 NUL；超出所需负载的字节被忽略。Decimal128 由 Rust 核心解码并保留支持的小数精度；marshal_bson_value 输出仍是字符串。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-bson-value" locale="zh-Hans" />

## 相关方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
