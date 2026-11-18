---
locale: zh-Hans
source_revision: b283786af672117d0c3017e513f97f5df201a379427acab824b383812aa4cf93
api_id: "config::ConfigValue::Decimal"
---
# config::ConfigValue::Decimal

选择 Decimal 配置输入。

```rust
Decimal (& 'a Decimal)
```

## 参数、返回值及常规用法

借用现有小数，在解码时克隆系数、scale 和初始化状态。源值仍可使用。浏览器源值在解码前释放或移动后，该借用变体不可用。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-decimal" locale="zh-Hans" />

## 浏览器边界

config 与 serde feature 已启用。源文本和字节最多 8192 个 UTF-8 字节；Rust 在解析前检查小数文本和 JSON 解码后的字符串，因此转义不能绕过 [资源限制](../resource-policy)。128 位整数必须使用范围内的 bigint。借用的 Decimal/NullDecimal 在解码时必须仍有效，转换生成独立值，不消耗输入。Serde 的通用 trait 语法由具体 JSON 适配器演示。

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
