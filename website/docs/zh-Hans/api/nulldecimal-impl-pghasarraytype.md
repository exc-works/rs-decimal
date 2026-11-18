---
locale: zh-Hans
source_revision: 329a8011aecf030884cb84a14d70c08abd0f17ed41927f957f56a638648169ac
api_id: "NullDecimal::impl PgHasArrayType"
---
# NullDecimal::impl PgHasArrayType

使用原生 SQLx PostgreSQL NUMERIC 集成。

```rust
impl PgHasArrayType for NullDecimal 
```

## 常规用法与注意事项

需要 postgres feature 和 SQLx 0.8 PostgreSQL 类型。此实现仅适用于原生环境：浏览器 Playground 不连接数据库。array_type_info() 返回 NUMERIC[] 的 OID 1231，使 SQLx 可以绑定数组。NullDecimal 数组元素通过有效标记保留 SQL NULL；完整数组的编解码由原生 SQLx 查询执行。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-pghasarraytype" locale="zh-Hans" />

## 执行方式

此条目是 Rust 原生 trait 语法，不提供同名 JavaScript 运算符或 SQLx 对象。 此示例由默认原生文档测试执行。

## 相关方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
