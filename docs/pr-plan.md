# rs-decimal PR 拆分

以下 PR 尚未创建。编号为执行顺序和依赖标识，各 PR 合并时更新公共 API 兼容矩阵与测试覆盖状态。完整目标见 [实现计划](implementation-plan.md)，接口细节见 [技术方案](technical-design.md)。

## PR 01 工程与兼容基线

依赖：无。建议标题：`chore: establish Rust crate and Go compatibility baseline`。

交付 Cargo 骨架、模块目录、基础 CI、MIT 许可及来源说明、固定 Go SHA、源契约与 fixtures 清单。导出 Go 公共 API 清单，记录 Rust 入口、返回类型、错误、scale、PR 归属；覆盖 SafeAdd/SafeSub、AppendText、MostSignificantBit 和框架辅助函数等容易遗漏项。无对应 Rust 接口的项目明确映射或延后。

兼容矩阵落在 `docs/api-compatibility.md`，字段与状态规则遵循技术方案的“API 兼容矩阵”。用 Go 语法树导出包含 BSON 条件入口的全部公共声明，对照稳定 API ID 检查缺失、重复和未知项；每项明确交付层次、PR 与处理方式。具体签名和测试尚待设计时保留显式状态，不能提前标成已兼容。

验收：cargo check/test、fmt、Clippy 通过；SHA 与文件校验和匹配；API 清单无漏项；确认依赖兼容性和暂定 MSRV；Git 作者与提交者均为 Skywalker Luke 指定身份。

## PR 02 差分执行基础

依赖：01。建议标题：`test: add versioned Go and Rust conformance protocol`。

交付 JSONL 协议、Go 参考程序、Rust 执行器框架、调度脚本、完整状态比较器、panic/error 分类和回归存储。Go 参考依赖固定 SHA，开发 checkout 路径可配置，CI 不依赖本机 ../decimal。

同时建立 `tools/go-oracle/` 独立模块与候选结果验证协议，以最小精确有理数案例跑通进程。按技术方案记录抽取来源，移除 testing.T 和生产 Decimal 依赖；正确候选必须通过，错误系数/scale/初始化状态必须失败，unsupported/uncertified 不得计为通过。后续数学 PR 逐步增加对应证书，不能把独立验证全部留到 PR 14。

验收：协议 schema 与 round-trip 测试；故意制造数值、scale、状态、错误类型差异时均检测失败；超时和进程异常不被当作成功。尚未实现的操作显式报告 unsupported，不能以空成功结果占位，也不能将其计为兼容通过。

## PR 03 类型与构造

依赖：02。建议标题：`feat: port decimal representation parsing and comparison`。

交付 Decimal、NullDecimal 状态结构、ErrorKind、基础构造、精确字符串解析、Clone、符号与状态检查、比较、Min/Max。初步紧凑输出只供验证，完整格式在 PR 10a。

验收：未初始化与初始化零、零解析归一化、尾零、科学记数、非法输入、解析边界；相等值不同表示；Min/Max 相等时保留第一项；比较大 scale 不无谓展开；访问器副本不泄漏内部可变状态。

## PR 04 舍入与 scale

依赖：03。建议标题：`feat: port exact rounding and scale operations`。

交付整数辅助层、七种舍入、Rescale 系列、Shift、Floor/Ceil/Truncate/Round、StripTrailingZeros、SignificantFigures。

验收：精确有理数 oracle 覆盖正负中点、奇偶、方向、Unnecessary；保留目标 scale 和零状态；原始模式转换的非法值失败；scale 溢出在 debug/release 一致。检查不能补尾零的 SignificantFigures 规则。

## PR 05 基础算术与幂

依赖：04。建议标题：`feat: port arithmetic with single final rounding`。

交付 Add/Sub/Raw/Safe、Mul/MulExact/MulWithPrec、Quo/QuoWithPrec、QuoRem/Mod、Power、整数部分和余数检查。

验收：全部模式的有理数验证与随机差分；二次舍入反例；除数符号；0^0、负幂、最小指数、精确中间边界；默认 scale；Safe 方法非负断言；所有输入不变。至少一组极端 scale 快路径回归。

## PR 06 整数有理数与浮点转换

依赖：05。建议标题：`feat: preserve exact conversion and floating point boundaries`。

交付 BigInt/BigRat、整数范围与 exact 标志、浮点最短文本输入、f32/f64 正确舍入输出。

验收：2^24/2^53 邻域、二进制中点、次正规数、最大有限数、溢出、带符号零、NaN/Infinity；f32 单次舍入；随机浮点位模式与大整数比的 oracle；终止/非终止有理数、输入不变。exact=false 的有效转换不能误判为失败。

## PR 07 根的精确舍入

依赖：06。建议标题：`feat: port certified square and integer roots`。

交付整数根上界、Newton、精确中点比较、Sqrt/ApproxRoot 与 WithPrec，保留预算和错误顺序。

验收：独立舍入单元证书；负奇数根、负偶数根；Sqrt/ApproxRoot 默认 scale 为 `max(input scale, 20)`，各覆盖输入 scale 为 0、20、30，与相同目标的 WithPrec 比较完整结果；WithPrec 另测 p=0、低于及高于输入 scale；高次根与预算边界；401 次、500 次根回归；根相关 must-succeed 全部成功。抽取 Go 根 oracle 后验证 Rust 候选，并拒绝篡改结果。

## PR 08 区间原语与 Ziv 驱动

依赖：04。建议标题：`feat: add outward intervals and rounding certification`。

交付 fraction、乘除、整数乘数、倒数所需原语、二进制定向缩放、端点舍入和 guard 增长。保持内部 API，不提前暴露不完整数学函数。

验收：精确有理数证明样本包含关系；负数和跨零区间；缓冲区不污染输入；强制精度重试案例；区间跨舍入边界时必须拒绝；预算耗尽不能返回近似成功。

## PR 09 对数和指数

依赖：06、08。建议标题：`feat: port certified logarithms and exponential`。

交付 Ln/Log2/Log10/Exp 及全部 WithPrec，保留精确快路径、参数归约、余项界与错误/panic 形态。若 diff 过大，拆成 09a 对数、09b 指数，两者各有独立验证。

验收：抽取的严格有理区间 oracle 验证 Rust 候选并拒绝篡改结果；Ln/Log2/Log10/Exp 默认 scale 为 `max(input scale, 20)`，各覆盖输入 scale 为 0、20、30，与相同目标的 WithPrec 比较完整结果；WithPrec 另测 p=0、低于及高于输入 scale；接近 1、2 的幂两侧、舍入跃迁两侧、大负指数；多精度差分；PR 07 和本 PR 均完成后全量运行固定 45 个 must-succeed 用例及独立证书；返回 NoConvergence 不能代替承诺成功。

## PR 10a 格式化

依赖：05。建议标题：`feat: preserve decimal formatting behavior`。

交付 Decimal 紧凑与尾零输出、AppendText、分隔符格式、Go 格式兼容入口、Display 文档。

验收：宽度、符号、精度、#、f/e/E/g/G 及现有其他 verb 清单；%g 舍入进位后选记数法；科学格式的大 scale 不展开零；AppendText 行为；全部格式不变更输入。

PR 01—09 和 10a 均完成后执行实现计划中的核心预览退出门槛，提前完成范围内的架构、构建模式、oracle、打包和文档检查。此检查点不依赖 10b—13，不自动发布 crate。

## PR 10b 文本协议与空值

依赖：10a。建议标题：`feat: preserve Text and Param decoding states`。

交付 Decimal/NullDecimal 的 Text/Param 入口及 NullDecimal.String，明确空值规则与原地更新语义。

验收：空文本与空白的差异；有效/无效 NullDecimal 和内部未初始化状态；成功状态转移、失败原子性；编码不修改输入。完整格式化验收沿用 10a。

## PR 11 JSON Serde 与空值

依赖：06、10b。建议标题：`feat: add JSON compatibility and Serde adapters`。

交付 JSON 原文数字解析、字符串编码、Decimal/NullDecimal 原地更新、Serde 对应适配和差异说明。serde/json 功能边界清晰。

验收：大于二进制浮点精度的数值 token 不丢位；null、空字符串、缺失字段与不同接收者初态；valid=true 且内部未初始化编码为有效零；所有失败保持状态；通用 Deserialize 与原地入口分别测试。

## PR 12 Go 二进制互通

依赖：04、10a。建议标题：`feat: support Go decimal binary encoding`。

交付 precision 前缀、big.Int Gob 载荷编码解码、Go 生成金样本和 Rust 反向编码样本。

验收：Go encode→Rust decode 和 Rust encode→Go decode；规范编码字节一致；去尾零、未初始化、空 bytes、错误版本、截断、precision 上限与失败原子性；32 位执行验证转换顺序。

## PR 13 YAML XML BSON 与通用扫描

依赖：11。建议拆为四个可分别审查的 PR：13a YAML、13b XML、13c BSON、13d Scan/Value。

交付原文/引号/tag 语义、XML 元素与属性、BSON String/Double/Int32/Int64/Decimal128/Null、Scan 输入枚举和输出规则。通用扫描不是某个数据库驱动的完成声明。

验收：复用 yaml_scalars；八进制、十六进制、带引号数字；XML 空白；BSON Decimal128 有限与特殊值；各入口 null 差异；所有失败原子性；单独启用各功能可编译和通过测试。

## PR 14 发布候选验证

依赖：01—13 全部完成（含 10a、10b）。这是协议兼容版的发布候选，不是核心预览的前置条件。建议标题：`test: enforce cross-language release compatibility gates`。

交付完整 API 状态矩阵、跨平台 CI、长期 fuzz 任务配置、独立 oracle 和回归报告、同语义性能基线、使用文档、包内容检查、版本与 MSRV 声明。

验收：所有承诺方法和 must-succeed 全通过；无未解释差分；debug/release 和实际运行的 32/64 位配对；默认/单独/全特性；cargo package 可在隔离目录验证。性能结论有结果核对和测量证据。本 PR 不自动执行 cargo publish。

## PR 15 后续生态集成

依赖：14；可在核心稳定后提前准备，但不阻塞数值核心预览。

按用户实际采用的数据库和 Web/配置框架选择适配对象，每个驱动或框架单独 PR。优先提供一个数据库实现和精确数值验证辅助层，再扩展配置解码与翻译消息。不要一次承诺所有 Rust 框架。

验收：真实数据库往返及 NULL、精度、拒绝范围测试；验证器不经浮点比较；配置解码保留数字原文。旧 Go 框架辅助 API 的每一项在矩阵中有明确去向。未来新增适配器不修改数值核心语义。

## 依赖与审查约定

主线为 01→02→03→04→05→06→07；08 在 04 后可准备，09 依赖 06 和 08；10a 在 05 后可准备，10b 依赖 10a；11 依赖 06 和 10b，12 依赖 04 和 10a，13 依赖 11。核心预览等待 01—09 与 10a 并执行独立门禁；14 等待全部范围内实现（含 10b），作为协议兼容版门禁。该依赖关系不意味着必须使用并行代理。

每个 PR 描述写清具体行为变化、对应 Go API、输入输出例子、验证证据和剩余差异。算法重构、数值语义变化与性能优化不合并为一个难以审查的 PR。已发现差分必须留下回归；不能修改期望值来迎合移植结果。

当前 PR 01—03 已审核通过；PR 04 已提交审核且 CI 通过；PR 05 已实现基础算术与幂，等待审核。后续 PR 尚未开始。每层 PR 以前一层为 base，审核与处理结果记录在对应 PR。
