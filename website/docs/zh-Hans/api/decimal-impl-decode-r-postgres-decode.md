---
locale: zh-Hans
source_revision: 21da10a6e76a84c5146b6a083d73a886edce7505fb47206e66d109e57f693aeb
api_id: "Decimal::impl Decode < 'r , Postgres >::decode"
---
# Decimal::impl Decode < 'r , Postgres >::decode

使用原生 SQLx PostgreSQL NUMERIC 集成。

```rust
fn decode (value : PgValueRef < 'r >) -> Result < Self , BoxDynError >
```

## 常规用法与注意事项

需要 postgres feature 和 SQLx 0.8 PostgreSQL 类型。此实现仅适用于原生环境：浏览器 Playground 不连接数据库。decode(PgValueRef) 返回 `Result<Self, BoxDynError>`。SQLx 提供借用的数据库值，不应伪造其私有传输表示。二进制及文本 NUMERIC 保留数值和显示 scale，包括带 scale 的零。拒绝 SQL NULL；可空列使用 NullDecimal 或 `Option<Decimal>`。非空结果会初始化。拒绝格式错误和非有限 NUMERIC，不做舍入。此示例在 CI 的专用 PostgreSQL 服务中执行，浏览器不需要外部数据库。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-decode-r-postgres-decode" locale="zh-Hans" />

## 执行方式

此条目是 Rust 原生 trait 语法，不提供同名 JavaScript 运算符或 SQLx 对象。 使用专用 PostgreSQL 服务和 DATABASE_URL，运行 `cargo run --locked -p decimal-web-examples --features postgres-live` 执行此示例。未启用该 profile 时会明确报告跳过，不能算作数据库验证通过。

## 相关方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
