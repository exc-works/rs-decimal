---
locale: zh-Hans
source_revision: 0830b6acb470b67cab11063b94daf8eafba63a697871936bf2d04b61331f067d
api_id: "NullDecimal::impl Type < Postgres >"
---
# NullDecimal::impl Type < Postgres >

使用原生 SQLx PostgreSQL NUMERIC 集成。

```rust
impl Type < Postgres > for NullDecimal 
```

## 常规用法与注意事项

需要 postgres feature 和 SQLx 0.8 PostgreSQL 类型。此实现仅适用于原生环境：浏览器 Playground 不连接数据库。type_info() 返回 NUMERIC 的 OID 1700。compatible(ty) 只接受该确切 OID，不接受 float8、整数或任意文本类型，不承诺隐式浮点转换。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-type-postgres" locale="zh-Hans" />

## 执行方式

此条目是 Rust 原生 trait 语法，不提供同名 JavaScript 运算符或 SQLx 对象。 此示例由默认原生文档测试执行。

## 相关方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
