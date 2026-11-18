---
locale: zh-Hans
source_revision: f24ab15a12ac3cc5edffa3bb6354648be7a4944a94929ac82037f54fe89c586d
api_id: "NullDecimal::impl Encode < '_ , Postgres >::encode_by_ref"
---
# NullDecimal::impl Encode < '_ , Postgres >::encode_by_ref

使用原生 SQLx PostgreSQL NUMERIC 集成。

```rust
fn encode_by_ref (& self , buf : & mut PgArgumentBuffer) -> Result < IsNull , BoxDynError >
```

## 常规用法与注意事项

需要 postgres feature 和 SQLx 0.8 PostgreSQL 类型。此实现仅适用于原生环境：浏览器 Playground 不连接数据库。encode_by_ref(&self, &mut PgArgumentBuffer) 返回 `Result<IsNull, BoxDynError>`。借用保留数值，成功编码后才向已有参数缓冲区追加字节。无效 nullable 返回 IsNull::Yes，不写字节。有效值编码内部 Decimal，即使内部零未初始化。二进制格式使用大端 base-10000 分组，保留显示 scale，不经过浮点数或 BigDecimal。scale 超过 16383 或整数位超过 131072 时拒绝；编码错误保留目标缓冲区。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-encode-postgres-encode-by-ref" locale="zh-Hans" />

## 执行方式

此条目是 Rust 原生 trait 语法，不提供同名 JavaScript 运算符或 SQLx 对象。 此示例由默认原生文档测试执行。

## 相关方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
