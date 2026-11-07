# 阅读与维护代码

## 从使用契约进入

先阅读 `src/lib.rs` 的可执行示例和类型说明，再查看具体方法的 rustdoc。`Decimal` 表示 `coefficient × 10^-scale`；Default 的未初始化零和 NullDecimal 的 valid 是独立状态。数值相等不表示表示状态相同，因此测试应在需要时同时核对系数、scale、初始化状态和 valid。

公开 API 保留 Go 兼容命名，内部代码优先使用能说明单位的名字，如 coefficient、scale、weight。对可直接读懂的语句不加重复注释；注释解释舍入证明、边界顺序、协议单位、错误原子性和资源限制。公共入口不能缺少文档，示例应可运行；返回值、精度单位、错误与 panic 必须和实现一致。

## 模块职责

| 模块 | 负责的内容 |
| --- | --- |
| decimal / integer | 表示、解析、比较与整数基础运算 |
| arithmetic / rounding | 精确系数运算、scale 对齐、唯一的有符号舍入规则 |
| conversion / formatting | 精确有理数和 IEEE 转换、文本输出 |
| interval / transcendental | 向外取整的区间，以及逐次提高精度的认证循环 |
| root / logarithm / exponential | 领域算法、参数约减、根中点判断与级数余项界 |
| text / binary / json / yaml / xml / bson | 协议边界及完整状态的原子更新 |
| sql / postgres | 无驱动 SQL 语义与 SQLx NUMERIC 的 base-10000 编码 |
| validation / validation::messages | 数值规则和消息渲染 / 独立存放的语言模板 |
| config | 显式输入类型、保留 JSON 原文的 Serde 字段适配 |

`raw()` 只借用系数；算术对齐借用相同 scale 的系数，只为确实需要移位的非零数分配。这样既避免重复克隆，也把“对齐不丢精度”的不变量放在一个位置。解析先完整验证语法，再走最多 18 位的 u64 路径；更长输入仍使用任意精度整数，不能把快速路径变成输入范围限制。

## 数值修改的验收

区间端点分别向下/向上舍入；只有两个端点在目标十进制网格得到同一舍入结果才返回成功。不得通过增加一个经验 epsilon、调用 float 近似或把 NoConvergence 当成功来优化。根运算直接比较精确中点的幂，避免浮点误差影响 ties-to-even。

行为改动先加入能区分旧行为与正确行为的回归例，再运行对应单元测试、全量 Go 差分和独立 oracle。性能变更应提供相同环境、相同输入的前后数据，同时保留进程内测量和端到端差分计时的区别。CI 运行 MSRV/stable、32/64 位配对、跨平台和 PostgreSQL；修改底层 PR 后，把修复同步到上层，重新审核最终 head。
