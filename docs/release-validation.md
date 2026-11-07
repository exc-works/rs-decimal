# 发布候选验证

当前完整实现覆盖 166 项基线 API（包括平台映射），全量固定差分语料为 21,579 例。版本 0.1.0-alpha.1 尚未发布。早期堆叠 PR 合并进了中间分支；PR #22 将完整实现集成到 main，是否已合并以 GitHub 当前状态为准。后续变更必须在各自最新提交重新通过 CI。

历史验证：2026-11-06，macOS arm64、Rust 1.99.1、Go 1.27.3，512 个固定随机用例、45 个 must-succeed 输出/状态及独立证书通过；20 秒随机验证通过 251,648 例和 188,624 个独立算术证书（seed=14598561）。这些是当时提交的证据，不替代后续提交检查。

CI 覆盖 MSRV 1.85/stable、全 feature、独立 feature、debug/release、打包产物、Linux 32/64 位 Go/Rust 配对、macOS/Windows 和真实 PostgreSQL。每周/手动随机任务保存 seed、失败请求和双方响应；cron 仅在工作流进入默认分支后生效。

协议失败保留完整接收者状态；二进制/BSON 双向互通、PostgreSQL 最大合法 NUMERIC/NULL/数组、原文配置 token 和精确验证规则均有测试。固定上游 11 份文件及 166 项声明清单由门禁核对。

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

发布步骤和独立消费者验证见 [发布流程](releasing.md)。兼容差异集中记录在 [API 矩阵](api-compatibility.md)。
