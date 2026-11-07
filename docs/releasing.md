# Alpha 发布流程

当前版本为 `0.1.0-alpha.1`，尚未发布。先将堆叠 PR 自底向上合并；每次合并前把下一层 base 改为 main，确认最终 main 包含所有层，再以该提交进行发布验证。

## 验证候选提交

1. 确认工作区干净、版本与 CHANGELOG 一致，CI 的 MSRV/stable、32/64 位差分、macOS/Windows、PostgreSQL 和文档检查全部通过。
2. 运行 `python3 tools/check-release-matrix.py`、全量差分和独立 oracle；手动运行扩展随机工作流，保存 seed 与结果工件。
3. 执行 `cargo doc --locked -p rs-decimal --all-features --no-deps` 和 `cargo test --locked -p rs-decimal --all-features --doc`。docs.rs 设置 all-features，所有可选入口都出现在文档中。
4. 执行 `cargo package --locked --list` 检查发布内容，然后 `cargo publish --locked -p rs-decimal --dry-run --all-features`。测试解包产物及 examples/configuration.rs，避免仓库相对路径依赖。
5. 执行 `python3 tools/check-consumer.py`，以无锁文件的独立 edition 2024 消费者验证依赖解析和全 feature；MSRV CI 用 Rust 1.85 执行同一命令。消费者使用 Cargo resolver 3 的 Rust 版本兼容解析。

## 经维护者决定后发布

重新查询 crates.io 的 rs-decimal 名称和发布账号权限；dry-run 不能预留名称或证明上传权限。确定源码提交和发布说明后，再发布并给相同提交创建 `v0.1.0-alpha.1` 标签/GitHub Release。不要把凭据写入仓库，不从未经验证的堆叠分支发布。当前 CI 只验证，不执行上传。

依据：[Cargo 发布文档](https://doc.rust-lang.org/cargo/reference/publishing.html)、[docs.rs 元数据](https://docs.rs/about/metadata)。
