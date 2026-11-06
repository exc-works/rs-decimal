# 核心预览验证

范围为 PR 01—09 和 10a。协议与生态适配继续实施，不在此检查点宣称完成；crate 未发布，发布前仍需核对 crates.io 名称可用性。

2026-11-06 本机 macOS arm64、Rust 1.99.1、Go 1.27.3：18,041 个固定差分、512 个固定种子随机用例及其独立证书通过，45 个 must-succeed 全部精确匹配并独立认证。debug/release 单元回归、fmt、Clippy、基线完整性、cargo package 与包内独立测试通过。包包含源码、Rust 测试、示例和许可，不包含 Go 工具或 fixtures。MSRV 1.85 在 CI 执行。

配对 32 位门禁在 Linux CI 原生执行 i686 Rust 和 GOARCH=386 Go（包含 debug/release 单元测试、全量差分、随机证书和固定 45 个成功证书）；64 位 Linux 使用同组门禁。两者上传 core-gates JSON 工件，以对应 PR 的实际成功记录为准。本机不伪造 32 位执行结果。

性能样本先比较每次输出再记录：512 个混合算术/根请求，包含子进程启动与 JSONL 编解码；三次中位数 Go 5.865 ms、Rust release 4.720 ms。本样本只建立可复跑的批量基线，不能解释为单个算法加速或跨机器性能承诺。重跑命令：

```sh
python3 tools/conformance/core-gates.py --go .cache/go-reference --rust target/release/decimal-conformance --oracle .cache/go-oracle
```

随机烟测种子为 0xDEC1A1；若失败，完整请求/结果写入 .cache/core-fuzz-failure.json。已知审核回归包括极端 scale 零除数、错误状态完整性、oracle 编码变异和根/对数的舍入边界，均进入常规测试。后续长期 fuzz 作业在 PR 14 扩展。
