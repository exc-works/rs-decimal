---
locale: zh-Hans
source_revision: 4748c543f00fa16cc184b2f3ac3218ce12c09982f49453b64f19d87393fb722a
api_id: "config::ConfigValue::Nullable"
---
# config::ConfigValue::Nullable

选择 Nullable 配置输入。

```rust
Nullable (& 'a NullDecimal)
```

## 参数、返回值及常规用法

借用现有 nullable。decode_nullable 克隆有效标志和隐藏数据；decode_decimal 只接受有效值并克隆内部 Decimal。解码前的后续修改会被观察到。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-nullable" locale="zh-Hans" />

## 浏览器边界

config 与 serde feature 已启用。源文本和字节最多 8192 个 UTF-8 字节；Rust 在解析前检查小数文本和 JSON 解码后的字符串，因此转义不能绕过 [资源限制](../resource-policy)。128 位整数必须使用范围内的 bigint。借用的 Decimal/NullDecimal 在解码时必须仍有效，转换生成独立值，不消耗输入。Serde 的通用 trait 语法由具体 JSON 适配器演示。

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
