# rs-decimal

rs-decimal 计划提供与 exc-works/decimal 数值和状态行为兼容的 Rust 实现。当前已建立 Rust crate 骨架与固定兼容基线；数值 API 尚未实现，crate 尚未发布。

## 开发

暂定 MSRV 为 Rust 1.85（edition 2024），CI 同时检查 MSRV 与 stable。核心依赖为固定版本的 num-bigint、num-integer、num-traits、num-rational。当前无默认 feature。

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
