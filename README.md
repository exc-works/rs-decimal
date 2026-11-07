# rs-decimal

rs-decimal 是保持 exc-works/decimal 数值和状态契约的 Rust 移植。数值核心、格式化及核心预览门禁已通过；Text/Param、JSON 与可选 Serde 已实现，其余协议和生态适配尚未完成，crate 尚未发布。

## 开发

MSRV 为 Rust 1.85（edition 2024），CI 同时检查 MSRV 与 stable。核心依赖为固定版本的 num-bigint、num-integer、num-traits、num-rational 和 ryu。当前无默认 feature。

```sh
cargo check --locked
cargo test --locked
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
python3 tools/check-baseline.py
```

[API 兼容矩阵](docs/api-compatibility.md)记录每个入口的实施与验证状态。原始契约和 fixtures 位于 `testdata/upstream/`，许可证与来源见 `LICENSE`、`NOTICE`。

## 实现文档

- [实现计划](docs/implementation-plan.md)：目标、范围、里程碑、风险和完成标准。
- [技术实现方案](docs/technical-design.md)：类型、算法、错误、转换、编解码和验证协议。
- [PR 拆分](docs/pr-plan.md)：依赖顺序、每个 PR 的交付内容与验收条件。

计划基线为 Go 仓库提交 `a8791208e2d15483cf652cd2814b74b0c1a84187`，采用该提交的 v0.8.0 数值契约。仓库名称确定为 `rs-decimal`；crates.io 同名包的可发布性尚待核实。

Git 仓库本地作者与提交者身份为 `Skywalker Luke <skywalkerO_0@proton.me>`。

## 使用数值核心

```rust
use rs_decimal::{Decimal, GoFormat, RoundingMode};
let price = Decimal::must_from_string("12.350");
assert_eq!(price.string_with_trailing_zeros(), "12.350");
let total = price.mul_exact(&Decimal::new(3));
assert_eq!(total.string(), "37.05");
let third = Decimal::new(1).quo_with_prec(&Decimal::new(3), 2, RoundingMode::HalfEven);
assert_eq!(third.string_with_trailing_zeros(), "0.33");
assert_eq!(Decimal::new(2).sqrt_with_prec(5)?.string(), "1.41421");
assert_eq!(price.format_go(GoFormat { verb: 'f', precision: Some(2), ..GoFormat::default() }), "12.35");
# Ok::<(), rs_decimal::DecimalError>(())
```

Decimal 保留系数和 scale；Default 为未初始化零，数值相等不代表表示相同。算术不改变输入。Ln/根等返回 Result，Log2 默认入口以及负精度、除零等契约使用 DecimalError 类型化 panic。七种舍入模式中 Up 为远离零，Ceiling 为向正无穷。

版本 `0.1.0-alpha.1` 尚未发布；预览范围、错误规则和验证证据见 API 矩阵。`tools/conformance/core-gates.py` 执行全量差分、固定种子的 512 个随机用例及独立 oracle，再核对每次结果并报告包含进程/JSONL 开销的批量计时；不据此承诺算术加速。CI 实际运行 i686 Go/Rust 配对和 64 位配对、debug/release 测试，保存计时报告。极端原生 scale fixture 在 32 位配对时实例化为 isize::MAX，i64/u64 数值范围保持不变。

## 可选 JSON 与 Serde

`json` 提供 marshal_json/unmarshal_json，直接保留 JSON 数字 token，适合高精度外部输入；普通 Decimal 的 null 不修改旧值，NullDecimal 的 null 清空状态。`serde` 提供 Serialize/Deserialize，使用精确字符串、整数和 null；新建对象的 null 为默认值，没有旧状态。通用 Deserialize 拒绝浮点输入；通过 serde_json 读取原文小数时使用 json 入口或将十进制值编码为字符串。两个功能可以单独启用。

`yaml` 使用保留原文的事件解析处理 scalar 文档和标签；marshal_yaml 返回可交给 YAML emitter 的字符串/可空字符串。Go 二进制互通通过 marshal_binary/unmarshal_binary，保留 wire 格式与失败原子性。

`xml` 提供原文元素解码与属性值解码；编码返回字符内容/属性值，由调用者的 XML emitter 负责名称及转义。空元素清空值，失败保持原状态。

`bson` 提供 BSON type byte/raw bytes 适配，支持 string、double、int32、int64、Decimal128 和 null，不依赖 MongoDB 客户端。

`ScanValue` 与 `scan/value` 提供无驱动依赖的 SQL 输入/输出语义；文本保持精度，SQL NULL 使用 NullDecimal 的 valid 状态，浮点 NaN/Infinity 被拒绝。
