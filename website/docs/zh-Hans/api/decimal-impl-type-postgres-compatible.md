---
locale: zh-Hans
source_revision: 08d21be3db40ef9491657e8f165b85a45fc054adfc77bf6c387cc33e2075f02e
api_id: "Decimal::impl Type < Postgres >::compatible"
---
# Decimal::impl Type < Postgres >::compatible

使用原生 SQLx PostgreSQL NUMERIC 集成。

```rust
fn compatible (ty : & PgTypeInfo) -> bool
```

## 常规用法与注意事项

需要 postgres feature 和 SQLx 0.8 PostgreSQL 类型。此实现仅适用于原生环境：浏览器 Playground 不连接数据库。type_info() 返回 NUMERIC 的 OID 1700。compatible(ty) 只接受该确切 OID，不接受 float8、整数或任意文本类型，不承诺隐式浮点转换。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-type-postgres-compatible" locale="zh-Hans" />

## 执行方式

此条目是 Rust 原生 trait 语法，不提供同名 JavaScript 运算符或 SQLx 对象。 此示例由默认原生文档测试执行。

## 相关方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
