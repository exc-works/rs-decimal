# PostgreSQL / SQLx

启用 `rs-decimal` 的 `postgres` feature，配合 SQLx **0.8.6** 的 postgres 与所选运行时/TLS feature。库只提供 Type/Encode/Decode/PgHasArrayType 实现，不创建连接、选择 TLS 策略或执行查询。测试工具使用 Tokio。

```toml
rs-decimal = { version = "0.1.0-alpha.1", features = ["postgres"] }
sqlx = { version = "0.8.6", default-features = false, features = ["postgres", "runtime-tokio"] }
```

上例版本尚未发布；仓库使用者将第一行改为本地 path 或指定 git revision。

```rust,ignore
let amount = rs_decimal::Decimal::must_from_string("9007199254740993.00000000000000001");
let loaded: rs_decimal::Decimal = sqlx::query_scalar("SELECT $1::numeric")
    .bind(&amount).fetch_one(&mut connection).await?;
assert_eq!(amount, loaded);
```

NUMERIC 按大端 base-10000 二进制组精确编码/解码，不经过 f32/f64 或其他 decimal 库。保留服务器 dscale，包括 0.0000；支持二进制与简单查询的文本返回、NUMERIC 数组以及 Option<Decimal>/NullDecimal。普通未初始化 Decimal 编码为数值零，读回为初始化零；无效 NullDecimal/None 编码为 SQL NULL，NULL 解码到普通 Decimal 会报错，解码到 NullDecimal 为无效默认状态。数据库本身不保存 Rust 的初始化标记。

输入 scale > 16383 返回 InvalidPrecision；整数位 > 131072 返回 Overflow。拒绝 NaN/Infinity 和非法 wire 数据。检查在写入参数缓冲区前完成，错误不追加部分 payload。支持合法最大密集值的 36,864 个 base-10000 组，组数按 PostgreSQL 的无符号 16 位字段处理。只声明 NUMERIC/NUMERIC[] 类型兼容，不自动将 float8/int8 列视为 NUMERIC；请在查询中显式转换。

使用不指定 precision/scale 的 NUMERIC 列可保持输入 scale。NUMERIC(p,s) 由服务器按列声明舍入和检查范围，这与 Rust 算术的 HalfEven 默认规则不同。范围和列语义依据 [PostgreSQL NUMERIC 文档](https://www.postgresql.org/docs/18/datatype-numeric.html)。SQLx 类型接口依据固定 0.8.6 源码；传递依赖锁定到 Rust 1.85 可编译版本，升级锁文件后必须重跑 MSRV 门禁。

真实验证命令：

```sh
DATABASE_URL=postgres://postgres@127.0.0.1:55439/postgres \
  cargo test --locked -p decimal-postgres-tests -- --ignored
```

此测试要求专用服务器，仅创建连接级临时表；缺少 DATABASE_URL 会失败，不静默跳过。常规 workspace 测试将其标为 ignored，CI 独立 postgres 作业提供 PostgreSQL 18 服务并显式执行。2026-11-06 本机隔离 PostgreSQL 18 容器已通过：大于 f64 精度的值、147,455 位最大密集值、scale=16383、越界编码拒绝、文本/二进制零 scale、NULL、有/无效 NullDecimal、数组和非有限数值拒绝。
