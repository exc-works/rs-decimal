---
locale: zh-Hans
source_revision: 45b2b4aa308ba6b40046aead6dddbc34fd9a06d38be18fb996c0bc76cff67656
api_id: Decimal::impl FromStr::Err
---
# Decimal::impl FromStr::Err

引用 Decimal 的 FromStr 实现所返回的错误类型。

```rust
type Err = DecimalError
```

## 常规用法及语义

关联类型 `<Decimal as FromStr>::Err` 就是 `DecimalError`。非法文本返回该错误，不会触发 panic。可以读取错误类别来区分语法错误及其他失败。这是 Rust 类型别名，不是 JavaScript 构造函数。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr-err" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
