# 随机验证与失败重放

构建 release 的 decimal-conformance、tools/go-reference 和 tools/go-oracle 后运行：

```sh
python3 tools/conformance/fuzz.py --go .cache/go-reference --rust target/release/decimal-conformance --oracle .cache/go-oracle --seconds 60 --seed 14598561
python3 tools/conformance/fuzz.py --go .cache/go-reference --rust target/release/decimal-conformance --oracle .cache/go-oracle --replay .cache/fuzz-failure.json
```

每批覆盖基础算术、奇数根、rescale、ln/log2/log10/exp 和解析；协议随机选择 JSON/YAML/XML/Go binary/BSON 的成功和已知畸形输入。对数包含接近 1 的输入，指数限定在 [-20, 20]，避免把任意资源耗尽当作预期成功。操作类型与预期结果在调用实现之前确定；两边共同失败不能代替成功。所有成功数值结果另交给独立 rational/interval oracle 认证。协议检查包括错误分类、系数、scale、初始化和 nullable 有效性，错误时必须保留接收者状态。

报告 `.cache/fuzz-report.json` 按操作/协议/结果记录覆盖数。失败工件保存完整请求和预期结果，可用 --replay 原样重放，不依赖未来的生成器版本。`--seconds` 是批次之间检查的最短运行时长；单个子进程有 `--timeout`（默认 60 秒），CI job 另有总超时。当前发生超时的进程没有完整 JSONL 输出时，其响应列表为空，不能据此推断已处理的前缀。

PostgreSQL 的私有 wire codec 单元测试覆盖 512 个确定性生成值、所有帧截断、特殊 sign、越界 dscale、尾随字节、越界 base-10000 digit 和隐含精度丢失。真实服务测试另覆盖最大合法 NUMERIC、NULL/数组和文本/二进制路径。资源边界测试验证过量精度拒绝及失败状态保留；它们不承诺所有任意长度输入都具有常数资源成本。

2026-11-07 本地扩展验证：seed=14598561，15 秒运行 21,112 例，其中 16,240 个数值结果独立认证，十种数值操作及五种协议的成功/失败均有覆盖。后续提交须以最新运行报告为准。
