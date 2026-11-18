---
locale: zh-Hans
source_revision: b8a64fe28faf9f699b61b2f86e331fd9c5b8333882ce85a4476ea07f56d57e1d
api_id: Decimal::impl FromStr::from_str
---
# Decimal::impl FromStr::from_str

通过 Rust 的 parse 和 FromStr 解析小数文本。

```rust
fn from_str (text : & str) -> Result < Self , Self :: Err >
```

## 常规用法及语义

返回 Decimal 或 DecimalError。语法及零值标准化与 new_from_string 相同。浏览器调用该具名方法；Rust trait 语法仅适用于原生 Rust。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr-from-str" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
