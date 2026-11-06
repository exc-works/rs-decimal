# rs-decimal 技术实现方案

设计目标是保持 [固定基线的行为契约](implementation-plan.md)，同时用 Rust 的所有权与类型系统表达数据边界。先实现等价算法和状态，再基于测量优化。

## 工程结构与依赖

初始使用一个库 crate，辅助测试程序放在 tools 中。计划结构如下，当前尚未创建这些实现文件。

```text
src/
  lib.rs              公共导出
  decimal.rs          类型、状态、构造、比较
  rounding.rs         七种舍入和商余数判定
  arithmetic.rs       加减乘除、余数、幂与 scale
  integer.rs          有向整数除法、位操作、十次幂
  conversion.rs       整数、有理数与浮点转换
  root.rs             整数根和精确中点认证
  interval.rs         向外舍入的区间原语
  transcendental.rs   Ziv 驱动
  logarithm.rs        Ln、Log2、Log10
  exponential.rs      Exp
  format.rs           紧凑、保留尾零及 Go 格式兼容入口
  error.rs            错误类别和 panic 载荷
  nullable.rs         NullDecimal
  codec/              Text、JSON、Binary、YAML、XML、BSON
tests/                契约、oracle、回归、特性集成
testdata/             固定来源数据与差分回归
tools/go-reference/   调用固定 Go 库的参考程序
tools/go-oracle/      从固定 Go 测试抽取的独立数学验证器
tools/conformance/    Rust 执行器与差分调度
```

基础候选为 num-bigint、num-integer、num-traits，num-rational 用于有理数接口或测试。选择相互兼容版本后锁定测试依赖，记录最低 Rust 版本；本机 Rust 1.99.1 不是自动承诺的 MSRV。首版使用 std，不同时承诺 no_std、C ABI 或 wasm 专项支持。

可选功能计划为 serde、json、yaml、xml、bson。Serde 是类型集成，json 是需要保留数字原文和状态规则的格式适配；两者不是同一个兼容层。具体格式库在对应 PR 中经过原文保留能力验证后选定。

## 数据表示

```rust
pub struct Decimal {
    coefficient: Option<BigInt>,
    scale: usize,
}

pub struct NullDecimal {
    pub decimal: Decimal,
    pub valid: bool,
}
```

Decimal 字段私有。None 表示未初始化，此时内部 scale 必须为 0；Some(0) 为初始化后的零，可携带非零 scale。Default 返回未初始化值。数值相等不要求表示相同，1.0 与 1.00 相等；测试必须额外比较 scale 和初始化状态。

scale 内部使用 usize，但公共兼容范围上限为 isize::MAX，对应同位宽 Go int 的非负范围。接受 precision/shift 的兼容接口使用 isize，以保留负参数的错误路径。中间 scale 运算使用 checked 运算或更宽整数；MulExact 等入口继续保留各自原有限制，不能统一限制成解析上限。

首版匹配同位宽 Go/Rust；32 位和 64 位有意不同的原生整数边界在 fixture 中标记。二进制编码的 u32 scale 保持平台无关，在转换为原生类型前检查。

算术接收 &self 和 &Decimal，输出新值。提供 Clone，不能实现 Copy。大整数访问器返回副本。缓存只读十次幂，临时缓冲区由运算独占。先使用直接持有 BigInt 的结构，不预先引入 Arc 或内部可变性。

PartialEq/Ord 依据数学值手工实现，不得 derive 成结构相等。若实现 Hash，必须满足不同 scale 的相等值有相同 hash。Max/Min 数值相等时返回第一操作数的表示，包括未初始化状态。常量用只读常量或构造函数表达，不复制 Go 全局变量可被重赋值的机制。

## API 与错误

公共兼容方法采用对应 snake_case 名称，例如 mul_with_prec、quo_with_prec、power、ln_with_prec。保留 Precision 对应的 precision()，文档明确它是 scale。SafeAdd/SafeSub 的含义是结果必须非负，不能误实现成无 panic 的 checked 算术。

七种舍入使用枚举。安全 Rust 无法构造非法枚举，原 Go 非法整数模式的 panic 通过显式原始值转换入口测试；API 矩阵标记此类类型层面的差异。

内部返回 Result，兼容外层按原方法保留 Result 或 panic 的失败形态；不要使用 catch_unwind 实现正常控制流。类型化 panic 载荷可用 panic_any。差分程序以 unwind 构建，在操作边界捕获并分类；生产 panic=abort 模式不承诺可恢复。

ErrorKind 映射原有哨兵类别。DecimalError 需要支持 matches(kind) 或等价查询，使解码错误可同时匹配 Unmarshal 与 InvalidFormat 等原因，不能仅保留最外层枚举。格式解析器的 source 错误可以保留，但不要求等同 Go 的具体类型。非负断言等原有非哨兵 panic 单独分类，不能错误归入数值溢出。

加减运算符可在对应命名方法稳定后增加。首版不实现缺少舍入参数的 Mul/Div trait，不擅自引入默认模式；可选 try_* API 在兼容核心完成后增加。

### API 兼容矩阵

PR 01 创建 `docs/api-compatibility.md`，从固定提交的 Go 声明导出初始清单；不能仅根据本方案列举的方法手工建立清单。纳入包级导出函数、类型、导出字段、常量、变量，以及 Decimal/NullDecimal 的导出方法；包含 `bson` 构建标签下的入口，排除测试与示例。包级函数与同名方法、Decimal 与 NullDecimal 方法分别建项。

每项固定记录以下字段；长规则可链接到同文件中的详细条目：

| 字段 | 内容 |
| --- | --- |
| 标识与来源 | 稳定 API ID、Go 符号、完整签名、固定 SHA 下的文件位置、构建条件 |
| Rust 映射 | 目标符号、参数与返回类型、feature；尚未确定的部分显式标为待设计 |
| 行为 | 数值与 scale、初始化/Valid 状态、输入不变性或接收者更新规则 |
| 失败 | Result/panic/转换 bool 的形态、错误类别集合、参数检查顺序 |
| 交付 | 核心/协议/生态层次、所属 PR、直接移植/平台映射/明确延后的处理方式及原因 |
| 验证 | 契约用例 ID、差分 operation/case_id、独立验证依据或不适用原因 |
| 状态 | 未开始、实现中、已验证或延后；已验证必须附通过证据 |

PR 01 合并前每项必须确定交付层次、PR 和处理方式；该阶段允许具体 Rust 签名和测试 ID 待设计，但不得写成已兼容。对应实现 PR 合并前补齐这些字段和行为规则。平台映射是处理方式，不自动代表验证通过；延后项必须明确范围和原因，不能计入已验证比例。发布时按对应层次检查每个承诺入口，不以全库汇总覆盖率代替逐项门禁。

PR 01 使用 Go 语法树提取声明并核对矩阵 ID 集合，同时检查重复项和未知项。Go 签名不能单独推导 panic、scale 或状态行为，这些字段须对照契约、实现和测试审查。后续 PR 更新映射与证据，基线未变时不得无说明删除来源项。

## 基础数值算法

| 运算 | 实现规则 |
| --- | --- |
| Add/Sub | 对齐至 max(s,t)，精确运算 |
| AddRaw/SubRaw | 改变系数，保留 scale |
| Mul | 精确系数乘积，直接舍入至 max(s,t) |
| MulExact | scale 为 s+t，保留原中间 scale 上限 |
| MulWithPrec | 直接舍入到 p，保留极小结果的无巨型十次幂快路径 |
| QuoWithPrec | 对系数分子/分母做精确十进制缩放，再一次 QuoRem 和舍入 |
| Quo | 调用目标 scale 为 max(s,t) 的路径 |
| QuoRem/Mod | 商向零截断，余数与被除数同号，保留规定 scale |
| Power | 精确中间幂，最终一次 HalfEven，保留 0^0 和负指数规则 |
| 比较 | 先符号、同 scale 和数量级界，必要时才精确对齐 |

舍入从精确余数与除数决定，使用 2*abs(remainder) 与 abs(divisor) 比较。禁止先算有限精度近似值再二次舍入。HalfUp/HalfDown 对负数分别是中点远离零/趋向零。

整数辅助层分别提供 trunc_div_rem、floor_div、ceil_div。Go big.Int.Div 是欧几里得除法，不能在负除数下直接等同 floor_div；逐调用点证明除数为正，其他情况显式适配。负数右移、位长和最低位奇偶也按实际数学用途验证，不依赖名称相似。

初始预算沿用原实现：解析精度数量级上限 1<<17；根迭代最多 300；根 degree、scale 及乘积受原预算约束；Ziv guard 从 64 到 8192 倍增。超越函数工作位数使用相同的有理上界换算。预算不是 universal scale 上限，精确快路径与预算检查顺序也必须匹配。

## 根与超越函数

Sqrt、ApproxRoot、Ln、Log2、Log10、Exp 的默认输出 scale 统一为 `max(input scale, 20)`，不是固定 20 位，也不是 20 位有效数字。WithPrec 变体严格使用指定的非负 scale `p`，允许 `p < 20`；成功结果均使用 HalfEven。该规则已核对固定基线的 `docs/numeric-contract.md`、`math.go` 中的 `workingLogPrec` 及六个默认入口。

每个默认入口都覆盖输入 scale 为 0、20、30 的用例，并与对应 WithPrec 在默认目标 scale 下比较系数、scale 和初始化状态；另测 WithPrec 的 `p=0`、小于输入 scale 和大于输入 scale。输入通过显式系数/scale 构造，避免字符串解析把零归一化。精确值、零结果与非精确结果的代表用例也保留相同状态检查。

根首先获得精确整数根下界，再通过精确中点幂决定最终系数。保留原前缀上界与 Newton 迭代，以减少首版成功/失败范围变化；仅 sqrt 的底层整数原语可替换为经过验证的等价实现。

区间存储整数上下界，共用 2^bits 工作 scale。每个原语维护包含关系；乘除、符号变换、平方和取倒数均必须验证端点方向。

Ln 保留二进制归约、倒数方向选择、atanh 级数和余项界。Log2/Log10 用已包围的对数比值，保留精确幂快路径。Exp 保留参数缩小、Taylor 余项、重复平方、负数倒数和极小结果认证零。

Ziv 仅当两个区间端点在目标 scale 下 HalfEven 结果相同才成功；预算耗尽返回 NoConvergence。不能以相邻迭代结果相同或 epsilon 取代认证。

## 转换与格式

浮点输入按原位宽的最短往返十进制语义解析，测试可能有多种最短表示时的选择与 scale；不能简单复用产生不同拼写或位数的 Display。NaN/Infinity 和负零按原入口处理。

浮点输出从精确整数比直接定位目标指数与有效位，以整数余数完成一次 ties-to-even；覆盖正常数、次正规数、舍入到零、最大有限数和溢出。f32 不经 f64。exact 通过数学等值判定，不能用往返文本相等判定。候选依赖若提供同语义实现，也必须通过这些门禁。

Display 输出紧凑 String；保留尾零通过显式方法。为 Go 的 f/e/E/g/G、宽度、符号和 # 等行为提供独立兼容格式入口，不假定 Rust 格式化语法能逐字表达 Go fmt。库存公共格式能力全部进入清单。%g 必须在舍入进位后选择记数法。

## 编解码与集成

所有原地解码先构造临时状态，成功后再提交。JSON null 对普通 Decimal 保留旧状态，对 NullDecimal 清空；通用 Deserialize 构造新对象时没有旧状态，必须区别描述并提供 in-place 方法。

JSON 数字必须读取原始 token，不经 f64。YAML 必须保留引号、tag 与数字原文，并匹配基线的八进制、十六进制等规则。XML 元素/属性、BSON 类型和值分别适配，不能只测试字符串往返。

Binary 保留 4 字节大端 precision 与 big.Int.GobEncode 载荷。复制对应 Go 实现规格和金样本，测试正负数、零、非法版本、截断、超限 precision 和非规范输入；不使用通用 Rust 序列化替换。编码前去尾零；空 bytes 解码为初始化零，未初始化值的 round-trip 不保持 IsNil。

Scan/Value 先用格式无关的输入枚举表达 nil、整数、浮点、文本和字节，保留原类型拒绝规则。SQL 驱动、验证、配置适配放在后续独立 PR，针对目标框架测试。NullDecimal 保留 valid=false 时内部数据仍可能存在的状态；不自动压缩成 Option<Decimal>。

## 差分协议与独立验证

使用 JSON Lines，一行一个请求/结果。协议固定 schema_version、case_id、operation、参数和输入完整状态。系数、scale、整数参数均编码为字符串，避免 JSON 消费端精度丢失；浮点用位宽和十六进制原始 bits；bytes 用 hex。未初始化值用 initialized=false，不能通过十进制文本代替。

响应包含 outcome=ok/error/panic、结果完整状态、error_kinds、原输入操作后状态，以及解码接收者前后状态。超时和进程崩溃由测试调度器分类为失败。stdout 只允许协议输出，诊断和 panic hook 输出走 stderr。

fixtures 包含源 SHA、架构和生成器版本。测试调度器以生成好的同一组输入喂给双方，不依赖两个语言的随机数发生器相同。每个失败保存最小化回归用例；有序错误类别集合比较不受错误字符串影响。

Go 参考执行器调用固定版本公开 API，Rust 执行器调用兼容层。差分输入按位长、scale、操作次数限定测试成本，不缩小库契约。大规模 fuzz 在隔离进程中执行，运行时间超限不得算 NoConvergence。

独立 oracle 用精确有理数和严格界，不调用生产舍入、区间或级数助手。固定 Go 基线中的根与超越函数 oracle 位于 `independent_oracle_test.go`，依赖 `testing.T`、包内 Decimal 状态及测试用舍入助手，不能直接作为普通 Go 包导入。

确定采用源码抽取方式：在 `tools/go-oracle/` 建立独立 Go 模块，将所需测试数学逻辑及其测试助手按依赖闭包复制，保留许可、来源 SHA、原文件校验和与改动说明。不修改上游 checkout，不导入生产 decimal 包；把 `testing.T` 失败改为显式验证结果，把 Decimal 参数改为系数、scale、initialized 等普通数据。有理数舍入助手同样来自独立测试逻辑，不借用生产舍入实现。抽取后的差异须单独审查，不能只凭 Go/Rust 输出一致宣称独立性。

oracle 通过独立 JSONL 进程接收 `schema_version`、`case_id`、operation、完整输入、舍入模式及 Rust 候选结果，数值字段沿用差分协议编码。它从输入独立构造精确有理数或严格区间，验证候选系数、目标 scale 和初始化状态；不会用 Go 参考程序的结果作为数学期望。响应为 `verified`、`mismatch`、`uncertified` 或 `unsupported`，附判定依据或失败原因。选定的 oracle 用例只允许 `verified` 通过；预算不足、未知操作、超时、崩溃和协议错误均不得计为通过。完整输入不变性和错误类别仍由契约/差分层验证；非数值接口在矩阵中注明 oracle 不适用及替代验证依据。

PR 02 建立进程协议与可执行的最小精确有理数验证，使用手写正确/错误候选进行验收；PR 04—06 增加舍入、算术和转换验证，PR 07 抽取根证书，PR 09 抽取对数/指数严格区间验证。每阶段都测试篡改系数、scale、初始化状态的拒绝路径，并保留中点奇偶和认证失败用例。PR 09 与 PR 07 均完成后，固定 45 个 must-succeed 用例全部送入 oracle；增加的默认 scale 回归作为独立补充，不删改固定语料。

后续可增加 Rust oracle，但不作为首版独立验证的前置条件。已有 500/1500 位参考文本是交叉检查，不独自构成最后一位的正确舍入证书。

## CI 与发布

每个 PR 运行格式、Clippy、单元、契约和对应差分测试。关键数值测试同时执行 debug/release；发布门禁执行支持的 64 位平台和配对的 32 位环境，32 位必须运行测试而非仅交叉编译。

可选特性按无默认、默认、每个独立特性、全特性检查，避免全特性掩盖缺失依赖。合并门禁使用固定随机种子；每个数值 fuzzer 建议至少 20 秒烟测，长时间运行放在定期任务。已失败 seeds 永久回归。

基准分别测解析、小金额、不同位长/scale 乘除、float、根和高精度数学，并先核对输出。性能报告注明 Go/Rust 版本、优化配置、平台及统计方法。不把算法等价推导为速度保证。

每次对外发布前执行 cargo package 与包内容检查，确保 fixtures、Go 参考程序和 oracle 的构建依赖不会污染使用者安装。MSRV、版本号、crate 名称可用性在首次核心预览发布前明确，协议兼容版发布候选再次核对；具体门槛见实现计划。实际发布是独立动作。

## 基础工具参考

- [num-bigint BigInt](https://docs.rs/num-bigint/latest/num_bigint/struct.BigInt.html)：大整数、位操作和转换原语。
- [num-integer Integer](https://docs.rs/num-integer/latest/num_integer/trait.Integer.html)：不同方向的整数除法。

上述链接用于能力核查；落地时固定依赖版本，不以 latest 文档替代版本兼容检查。
