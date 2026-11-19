---
locale: zh-Hans
source_revision: c96f97d1831472af0c5d9cbfe01f313b01533a770c9ed531016eff9474a327fd
api_id: "Decimal::impl Sub < & Decimal >::Output"
---
# Decimal::impl Sub < & Decimal >::Output

使用借用式 Rust Sub 语法。

```rust
type Output = Decimal
```

## 常规用法与注意事项

为 &Decimal 实现，操作数仍可使用。精确结果使用较大的操作数 scale，结果已初始化。关联 Output 为 Decimal。允许负结果，不应用 safe_add/safe_sub 的非负结果要求。trait 委托同一核心具名方法。浏览器不重载 JavaScript 运算符，应调用 add/sub/neg。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-sub-decimal-output" locale="zh-Hans" />

## 执行方式

此条目是 Rust 原生 trait 语法，不提供同名 JavaScript 运算符或 SQLx 对象。 此示例由默认原生文档测试执行。

## 相关方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
