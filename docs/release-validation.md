# 协议兼容版发布候选验证

2026-11-06，本机 macOS arm64、Rust 1.99.1、Go 1.27.3：21,291 个固定跨语言差分全部通过，512 个固定随机用例独立认证，45 个 must-succeed 输出/状态和独立证书全部通过。另执行 20 秒随机验证，251,648 个用例、188,624 个独立算术证书通过，seed=14598561。所有成功预期预先由生成器确定，不接受两边共同失败来代替成功。

160 个核心/协议 API 项已实现并有证据；6 个 Go 框架 API 明确分配到 PR 15 的 PostgreSQL SQLx、Serde 配置和精确验证层。check-release-matrix.py 拒绝其余入口存在待实现/待设计状态。协议入口原地失败保留完整 Decimal 和 nullable 状态；二进制及 BSON 双向互通通过。固定上游 11 份文件和 166 项声明清单完整性通过。

Rust 1.85 MSRV Clippy、全 feature 单元测试、cargo package 和打包产物自带测试均通过。Linux CI 实际运行 32/64 位 Go/Rust 配对、debug/release、独立 feature 与全 feature。新增 macOS/Windows 配对作业；它们的远端执行结论以本 PR 最新 CI 为准。长期任务配置每周/手动执行 900 秒随机验证，使用运行 ID 作为 seed，失败保存完整请求与双方响应；工作流在合并到默认分支后才按 cron 触发。

## 可复跑性能基线

以下中位数来自每组 3 次、每次均核对完整结果的 Go/Rust release 批量调用，包含子进程启动和 JSONL 开销，不是纯算法微基准，不作 Rust 普遍更快的承诺。表示/解析组包含高位数文本，Rust 当前明显较慢；这是后续优化基线，不改数值语义来换速度。

| 输入组 | 个数 | Go 中位 ms | Rust 中位 ms |
| --- | ---: | ---: | ---: |
| representation | 128 | 5.226 | 25.639 |
| arithmetic | 128 | 3.195 | 2.428 |
| conversion | 128 | 2.990 | 2.254 |
| roots | 111 | 2.931 | 2.245 |
| transcendentals | 128 | 3.567 | 3.996 |
| formatting | 128 | 3.044 | 2.157 |
| json | 128 | 3.283 | 2.599 |
| binary | 124 | 2.929 | 2.381 |
| yaml | 128 | 3.683 | 2.661 |
| xml | 128 | 3.359 | 2.452 |
| bson | 128 | 3.907 | 3.938 |
| sql | 128 | 3.166 | 2.628 |

复跑：`python3 tools/conformance/benchmark.py --go .cache/go-reference --rust target/release/decimal-conformance`。CI 将 core-gates、fuzz 与 benchmark JSON 保存为工件。计时不设跨机器绝对阈值；正确性失败始终阻止通过。

版本保持 0.1.0-alpha.1，MSRV 1.85，尚未发布或合并。后续框架入口在各自独立 PR 中验证真实数据库、精确配置 token 和验证错误消息。已说明的兼容差异集中在 API 矩阵：极端零除数先报错防资源耗尽、Rust 格式/Serde 类型系统、XML 文档与回调边界、Go 框架的 Rust 映射。
