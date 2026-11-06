# API 兼容矩阵

固定基线：`a8791208e2d15483cf652cd2814b74b0c1a84187`。PR 03 已实现类型、基础构造、解析和比较；仅标为已验证的条目通过了当前测试，其他入口仍未完成。完整 Go 签名、来源位置和构建条件见 [声明清单](api-inventory.json)。状态、字段和验收规则见 [技术方案](technical-design.md#api-兼容矩阵)。

PR 01 确定交付归属和处理方式；Rust 签名、逐入口行为细节、失败顺序及测试证据在对应实现 PR 中补齐。表内“待设计”不可计为兼容通过。核心入口遵循 [数值契约](../testdata/upstream/docs/numeric-contract.md)，解码入口必须另列 null 与失败状态规则。

| Go API ID | Rust 映射/签名 | 层次 | PR | 处理方式 | 数值/scale/状态 | 失败形态/顺序 | 验证证据 | 状态 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Between` | `between(v: &Decimal, lower: &Decimal, upper: &Decimal) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal` | `Decimal { coefficient: Option<BigInt>, scale: usize }`（私有字段） | 核心 | 03 | 直接移植 | Default 未初始化零；Valid 独立于内部 Decimal | 由类型和构造器保持内部不变量 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Abs` | `Decimal::abs(&self) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Add` | `Decimal::add(&self, other: &Self) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.AddRaw` | `Decimal::add_raw(&self, value: i64) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.AppendText` | Decimal::append_text（签名待设计） | 核心 | 10a | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.AppendTextWithTrailingZeros` | Decimal::append_text_with_trailing_zeros（签名待设计） | 核心 | 10a | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.ApproxRoot` | Decimal::approx_root（签名待设计） | 核心 | 07 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.ApproxRootWithPrec` | Decimal::approx_root_with_prec（签名待设计） | 核心 | 07 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.BigInt` | `Decimal::big_int(&self) -> BigInt` | 核心 | 06 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.BigRat` | Decimal::big_rat（签名待设计） | 核心 | 06 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.BitLen` | Decimal::bit_len（签名待设计） | 核心 | 06 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Ceil` | `Decimal::ceil(&self) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.CeilWithPrec` | `Decimal::ceil_with_prec(&self, prec: isize) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.Clone` | `Decimal::clone(&self) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Cmp` | `Decimal::cmp(&self, other: &Self) -> i32` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Equal` | `Decimal::equal(&self, other: &Self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Exp` | Decimal::exp（签名待设计） | 核心 | 09 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.ExpWithPrec` | Decimal::exp_with_prec（签名待设计） | 核心 | 09 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Float32` | Decimal::float32（签名待设计） | 核心 | 06 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Float64` | Decimal::float64（签名待设计） | 核心 | 06 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Floor` | `Decimal::floor(&self) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.FloorWithPrec` | `Decimal::floor_with_prec(&self, prec: isize) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.Format` | Decimal::format（签名待设计） | 核心 | 10a | 平台映射：独立 Go 格式入口 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.FormatWithSeparators` | Decimal::format_with_separators（签名待设计） | 核心 | 10a | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.GT` | `Decimal::gt(&self, other: &Self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.GTE` | `Decimal::gte(&self, other: &Self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.HasFraction` | `Decimal::has_fraction(&self) -> bool` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.Int64` | Decimal::int64（签名待设计） | 核心 | 06 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.IntPart` | `Decimal::int_part(&self) -> BigInt` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.IsInteger` | `Decimal::is_integer(&self) -> bool` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.IsNegative` | `Decimal::is_negative(&self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.IsNil` | `Decimal::is_nil(&self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.IsNotZero` | `Decimal::is_not_zero(&self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.IsPositive` | `Decimal::is_positive(&self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.IsZero` | `Decimal::is_zero(&self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.LT` | `Decimal::lt(&self, other: &Self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.LTE` | `Decimal::lte(&self, other: &Self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Ln` | Decimal::ln（签名待设计） | 核心 | 09 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.LnWithPrec` | Decimal::ln_with_prec（签名待设计） | 核心 | 09 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Log10` | Decimal::log10（签名待设计） | 核心 | 09 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Log10WithPrec` | Decimal::log10_with_prec（签名待设计） | 核心 | 09 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Log2` | Decimal::log2（签名待设计） | 核心 | 09 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Log2WithPrec` | Decimal::log2_with_prec（签名待设计） | 核心 | 09 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.MarshalBSONValue` | Decimal::marshal_bson_value（签名待设计） | 协议 | 13c | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.MarshalBinary` | Decimal::marshal_binary（签名待设计） | 协议 | 12 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.MarshalJSON` | Decimal::marshal_json（签名待设计） | 协议 | 11 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.MarshalText` | Decimal::marshal_text（签名待设计） | 协议 | 10b | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.MarshalXML` | Decimal::marshal_xml（签名待设计） | 协议 | 13b | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.MarshalXMLAttr` | Decimal::marshal_xml_attr（签名待设计） | 协议 | 13b | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.MarshalYAML` | Decimal::marshal_yaml（签名待设计） | 协议 | 13a | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Max` | `Decimal::max(a: &Decimal, b: &Decimal) -> Decimal` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Min` | `Decimal::min(a: &Decimal, b: &Decimal) -> Decimal` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Mod` | `Decimal::r#mod(&self, other: &Self) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.Mul` | `Decimal::mul(&self, other: &Self, mode: M) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.MulDown` | `Decimal::mul_down(&self, other: &Self) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.MulExact` | `Decimal::mul_exact(&self, other: &Self) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.MulWithPrec` | `Decimal::mul_with_prec(&self, other: &Self, prec: isize, mode: M) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.MustNonNegative` | `Decimal::must_non_negative(&self) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.Neg` | `Decimal::neg(&self) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.NotEqual` | `Decimal::not_equal(&self, other: &Self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Power` | `Decimal::power(&self, power: i64) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.Precision` | `Decimal::precision(&self) -> isize` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Quo` | `Decimal::quo(&self, other: &Self, mode: M) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.QuoDown` | `Decimal::quo_down(&self, other: &Self) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.QuoRem` | `Decimal::quo_rem(&self, other: &Self) -> (Self, Self)` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.QuoWithPrec` | `Decimal::quo_with_prec(&self, other: &Self, prec: isize, mode: M) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.Remainder` | `Decimal::remainder(&self) -> (BigInt, BigInt)` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.Rescale` | `Decimal::rescale(&self, prec: isize, mode: RoundingMode) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.RescaleDown` | `Decimal::rescale_down(&self, prec: isize) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.Round` | `Decimal::round(&self) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.RoundWithPrec` | `Decimal::round_with_prec(&self, prec: isize) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.SafeAdd` | `Decimal::safe_add(&self, other: &Self) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.SafeSub` | `Decimal::safe_sub(&self, other: &Self) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.Scan` | Decimal::scan（签名待设计） | 协议 | 13d | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Shift` | `Decimal::shift(&self, places: isize) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.Sign` | `Decimal::sign(&self) -> i32` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.SignificantFigures` | `Decimal::significant_figures(&self, figures: isize, mode: RoundingMode) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.Sqrt` | Decimal::sqrt（签名待设计） | 核心 | 07 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.SqrtWithPrec` | Decimal::sqrt_with_prec（签名待设计） | 核心 | 07 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.String` | Decimal::string（签名待设计） | 核心 | 10a | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.StringWithTrailingZeros` | Decimal::string_with_trailing_zeros（签名待设计） | 核心 | 10a | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.StripTrailingZeros` | `Decimal::strip_trailing_zeros(&self) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.Sub` | `Decimal::sub(&self, other: &Self) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.SubRaw` | `Decimal::sub_raw(&self, value: i64) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.Truncate` | `Decimal::truncate(&self) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.TruncateWithPrec` | `Decimal::truncate_with_prec(&self, prec: isize) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.Uint64` | Decimal::uint64（签名待设计） | 核心 | 06 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.UnmarshalBSONValue` | Decimal::unmarshal_bson_value（签名待设计） | 协议 | 13c | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.UnmarshalBinary` | Decimal::unmarshal_binary（签名待设计） | 协议 | 12 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.UnmarshalJSON` | Decimal::unmarshal_json（签名待设计） | 协议 | 11 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.UnmarshalParam` | Decimal::unmarshal_param（签名待设计） | 协议 | 10b | 平台映射：保留输入规则，不实现 Gin 接口 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.UnmarshalText` | Decimal::unmarshal_text（签名待设计） | 协议 | 10b | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.UnmarshalXML` | Decimal::unmarshal_xml（签名待设计） | 协议 | 13b | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.UnmarshalXMLAttr` | Decimal::unmarshal_xml_attr（签名待设计） | 协议 | 13b | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.UnmarshalYAML` | Decimal::unmarshal_yaml（签名待设计） | 协议 | 13a | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Value` | Decimal::value（签名待设计） | 协议 | 13d | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `DecodeHook` | 待设计 | 生态 | 15 | 明确延后：由 Rust 验证/配置适配器承接，Go 框架类型不直接移植 | 待设计 | 待设计 | 待实现 | 延后 |
| `DefaultGoPlaygroundValidatorTranslationMessages` | 待设计 | 生态 | 15 | 明确延后：由 Rust 验证/配置适配器承接，Go 框架类型不直接移植 | 待设计 | 待设计 | 待实现 | 延后 |
| `ErrDivideByZero` | `ErrorKind::DivideByZero` | 核心 | 03 | 直接移植 | 稳定类别声明；可通过 matches 查询，允许多类别 | 声明本身不失败；具体入口失败行为随相应 PR 验证 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `ErrInvalidArgument` | `ErrorKind::InvalidArgument` | 核心 | 03 | 直接移植 | 稳定类别声明；可通过 matches 查询，允许多类别 | 声明本身不失败；具体入口失败行为随相应 PR 验证 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `ErrInvalidFormat` | `ErrorKind::InvalidFormat` | 核心 | 03 | 直接移植 | 稳定类别声明；可通过 matches 查询，允许多类别 | 声明本身不失败；具体入口失败行为随相应 PR 验证 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `ErrInvalidLog` | `ErrorKind::InvalidLog` | 核心 | 03 | 直接移植 | 稳定类别声明；可通过 matches 查询，允许多类别 | 声明本身不失败；具体入口失败行为随相应 PR 验证 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `ErrInvalidPrecision` | `ErrorKind::InvalidPrecision` | 核心 | 03 | 直接移植 | 稳定类别声明；可通过 matches 查询，允许多类别 | 声明本身不失败；具体入口失败行为随相应 PR 验证 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `ErrInvalidRoot` | `ErrorKind::InvalidRoot` | 核心 | 03 | 直接移植 | 稳定类别声明；可通过 matches 查询，允许多类别 | 声明本身不失败；具体入口失败行为随相应 PR 验证 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `ErrNegativeRoot` | `ErrorKind::NegativeRoot` | 核心 | 03 | 直接移植 | 稳定类别声明；可通过 matches 查询，允许多类别 | 声明本身不失败；具体入口失败行为随相应 PR 验证 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `ErrNoConvergence` | `ErrorKind::NoConvergence` | 核心 | 03 | 直接移植 | 稳定类别声明；可通过 matches 查询，允许多类别 | 声明本身不失败；具体入口失败行为随相应 PR 验证 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `ErrOverflow` | `ErrorKind::Overflow` | 核心 | 03 | 直接移植 | 稳定类别声明；可通过 matches 查询，允许多类别 | 声明本身不失败；具体入口失败行为随相应 PR 验证 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `ErrRoundUnnecessary` | `ErrorKind::RoundUnnecessary` | 核心 | 03 | 直接移植 | 稳定类别声明；可通过 matches 查询，允许多类别 | 声明本身不失败；具体入口失败行为随相应 PR 验证 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `ErrUnmarshal` | `ErrorKind::Unmarshal` | 核心 | 03 | 直接移植 | 稳定类别声明；可通过 matches 查询，允许多类别 | 声明本身不失败；具体入口失败行为随相应 PR 验证 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Hundred` | `hundred() -> Decimal` | 核心 | 03 | 平台映射：只读构造函数 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Max` | `max(a: &Decimal, b: &Decimal) -> Decimal` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Min` | `min(a: &Decimal, b: &Decimal) -> Decimal` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `MostSignificantBit` | most_significant_bit（签名待设计） | 核心 | 06 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `MustFromString` | `Decimal::must_from_string(text: &str) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `New` | `Decimal::new(value: i64) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NewFromBigInt` | `Decimal::new_from_big_int(value: &BigInt) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NewFromBigIntWithPrec` | `Decimal::new_from_big_int_with_prec(value: &BigInt, prec: isize) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NewFromBigRat` | new_from_big_rat（签名待设计） | 核心 | 06 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NewFromBigRatWithPrec` | new_from_big_rat_with_prec（签名待设计） | 核心 | 06 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NewFromDecimal` | `Decimal::new_from_decimal(value: &Self) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NewFromFloat32` | new_from_float32（签名待设计） | 核心 | 06 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NewFromFloat64` | new_from_float64（签名待设计） | 核心 | 06 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NewFromInt` | `Decimal::new_from_int(value: isize) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NewFromInt64` | `Decimal::new_from_int64(value: i64, prec: isize) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NewFromString` | `Decimal::new_from_string(text: &str) -> Result<Self, DecimalError>` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NewFromUint64` | `Decimal::new_from_uint64(value: u64, prec: isize) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NewFromUintWithAppendPrec` | `Decimal::new_from_uint_with_append_prec(value: u64, prec: isize) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NewNullDecimal` | `NullDecimal::new(decimal: Decimal) -> Self` | 协议 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NewWithAppendPrec` | `Decimal::new_with_append_prec(value: i64, prec: isize) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NewWithPrec` | `Decimal::new_with_prec(value: i64, prec: isize) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NullDecimal` | `NullDecimal { pub decimal: Decimal, pub valid: bool }` | 协议 | 03 | 直接移植 | Default 未初始化零；Valid 独立于内部 Decimal | 由类型和构造器保持内部不变量 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NullDecimal.Decimal` | `decimal: Decimal` | 协议 | 03 | 直接移植 | Default 未初始化零；Valid 独立于内部 Decimal | 由类型和构造器保持内部不变量 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NullDecimal.MarshalBSONValue` | NullDecimal::marshal_bson_value（签名待设计） | 协议 | 13c | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NullDecimal.MarshalJSON` | NullDecimal::marshal_json（签名待设计） | 协议 | 11 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NullDecimal.MarshalText` | NullDecimal::marshal_text（签名待设计） | 协议 | 10b | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NullDecimal.MarshalXML` | NullDecimal::marshal_xml（签名待设计） | 协议 | 13b | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NullDecimal.MarshalXMLAttr` | NullDecimal::marshal_xml_attr（签名待设计） | 协议 | 13b | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NullDecimal.MarshalYAML` | NullDecimal::marshal_yaml（签名待设计） | 协议 | 13a | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NullDecimal.Scan` | NullDecimal::scan（签名待设计） | 协议 | 13d | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NullDecimal.String` | NullDecimal::string（签名待设计） | 协议 | 10b | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NullDecimal.UnmarshalBSONValue` | NullDecimal::unmarshal_bson_value（签名待设计） | 协议 | 13c | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NullDecimal.UnmarshalJSON` | NullDecimal::unmarshal_json（签名待设计） | 协议 | 11 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NullDecimal.UnmarshalParam` | NullDecimal::unmarshal_param（签名待设计） | 协议 | 10b | 平台映射：保留输入规则，不实现 Gin 接口 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NullDecimal.UnmarshalText` | NullDecimal::unmarshal_text（签名待设计） | 协议 | 10b | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NullDecimal.UnmarshalXML` | NullDecimal::unmarshal_xml（签名待设计） | 协议 | 13b | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NullDecimal.UnmarshalXMLAttr` | NullDecimal::unmarshal_xml_attr（签名待设计） | 协议 | 13b | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NullDecimal.UnmarshalYAML` | NullDecimal::unmarshal_yaml（签名待设计） | 协议 | 13a | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `NullDecimal.Valid` | `valid: bool` | 协议 | 03 | 直接移植 | Default 未初始化零；Valid 独立于内部 Decimal | 由类型和构造器保持内部不变量 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NullDecimal.Value` | NullDecimal::value（签名待设计） | 协议 | 13d | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `One` | `one() -> Decimal` | 核心 | 03 | 平台映射：只读构造函数 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `PrecisionFixedSize` | 待设计 | 协议 | 12 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `RegisterGoPlaygroundValidator` | 待设计 | 生态 | 15 | 明确延后：由 Rust 验证/配置适配器承接，Go 框架类型不直接移植 | 待设计 | 待设计 | 待实现 | 延后 |
| `RegisterGoPlaygroundValidatorTranslations` | 待设计 | 生态 | 15 | 明确延后：由 Rust 验证/配置适配器承接，Go 框架类型不直接移植 | 待设计 | 待设计 | 待实现 | 延后 |
| `RegisterGoPlaygroundValidatorTranslationsWithMessages` | 待设计 | 生态 | 15 | 明确延后：由 Rust 验证/配置适配器承接，Go 框架类型不直接移植 | 待设计 | 待设计 | 待实现 | 延后 |
| `RoundCeiling` | `RoundingMode::Ceiling` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `RoundDown` | `RoundingMode::Down` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `RoundHalfDown` | `RoundingMode::HalfDown` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `RoundHalfEven` | `RoundingMode::HalfEven` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `RoundHalfUp` | `RoundingMode::HalfUp` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `RoundUnnecessary` | `RoundingMode::Unnecessary` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `RoundUp` | `RoundingMode::Up` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `RoundingMode` | `RoundingMode`；`from_raw(i32) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Ten` | `ten() -> Decimal` | 核心 | 03 | 平台映射：只读构造函数 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `TranslateGoPlaygroundValidationErrors` | 待设计 | 生态 | 15 | 明确延后：由 Rust 验证/配置适配器承接，Go 框架类型不直接移植 | 待设计 | 待设计 | 待实现 | 延后 |
| `Zero` | `zero() -> Decimal` | 核心 | 03 | 平台映射：只读构造函数 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |

## PR 03 行为规则

- `Decimal::default()` 是未初始化零；显式整数构造初始化值并保留目标 scale。带 AppendPrec 的构造保持整数数学值并补零。BigInt 参数复制，访问器返回副本。Rust 引用排除了 Go 的 nil BigInt 参数；该输入属于类型层面的映射差异。
- NewFromString 精确解析 Go 契约中的十进制/科学记数，去两端 Unicode 空白，要求小数点两侧均有数字；指数为有符号 32 位整数，最终 scale 的绝对值不超过 131072。解析零归一化为初始化零、scale=0。失败返回 InvalidFormat；MustFromString 使用相同错误的类型化 panic。
- 显式 precision 为负首先 panic InvalidPrecision；非负原生范围内 precision 不统一受解析上限限制。原生有符号/无符号输入由 Rust 类型约束。
- Clone/NewFromDecimal 保留初始化状态与 scale；Abs/Neg 将未初始化零初始化，保留已初始化值的 scale。Sign/Is* 与比较不修改输入；nil 数学上等于零。
- Cmp 返回 -1/0/1，Eq/Ord 按数学值。Min/Max 相等时保留第一操作数完整状态；Between 包含两端且不自动交换上下界。极端 scale 比较先用位长界，不无条件展开十次幂。
- 数值检查和比较无额外语义错误；大整数分配仍受机器资源限制。常量映射为 zero/one/ten/hundred 构造函数，不提供可重赋值的全局变量。
- NullDecimal::new 将 valid 设为 true 并保留传入 Decimal；字段允许 valid=false 时内部仍有值。
- 初步 string/Display 仅用于调试与验证，完整格式化兼容仍属于 PR 10a。

## PR 04 行为规则

- 七种模式的整数值为 Down=0、Up=1、Ceiling=2、HalfUp=3、HalfDown=4、HalfEven=5、Unnecessary=6。`from_raw` 对非法值使用断言 panic；类型化 API 不可能接收非法枚举。
- Rescale/RescaleDown 和 Floor/Ceil/Truncate/Round 的 WithPrec 版本先拒绝负 precision（InvalidPrecision panic），再按目标 scale 构造初始化结果。无参数方法目标 scale=0。Unnecessary 只在需要舍入时 panic RoundUnnecessary。
- 舍入基于精确商余数，负中点 HalfUp 远离零、HalfDown 趋向零、HalfEven 取偶数系数。Floor 向负无穷，Ceil 向正无穷。
- Shift 保持精确值乘 10^places；places=0 保留 scale 并初始化 nil。非零 shift 的非负目标 scale 超过 i32::MAX 时 panic Overflow；负目标 scale 转为整数系数、scale=0。
- StripTrailingZeros 不减至负 scale；零归一化为初始化零、scale=0。SignificantFigures 的 figures<=0 为断言 panic；不为达到位数补零，零保留原 scale 并初始化 nil。
- 算术输入均不变。独立 Rust 邻点测试覆盖 22,736 个舍入组合；Go oracle 从上游独立测试抽取，来源及变更见 [PROVENANCE](../tools/go-oracle/PROVENANCE.md)。

## PR 05 行为规则

- Add/Sub 精确对齐到 max 输入 scale；AddRaw/SubRaw 修改系数并保留 scale。SafeAdd/SafeSub 和 MustNonNegative 对负结果使用断言 panic，成功结果初始化。
- Mul/Quo 的默认 scale=max 输入 scale；MulExact 使用两者之和，超过 i32::MAX 时先 panic Overflow（包括零输入）。MulWithPrec 不继承这一中间 scale 上限，直接在目标 scale 舍入。所有舍入只发生在最终目标；Unnecessary 的非精确结果 panic RoundUnnecessary。
- WithPrec 首先拒绝负 precision（InvalidPrecision）；Quo 接着拒绝零除数（DivideByZero），零被除数随后直接返回目标 scale 的零。非零除法的缩放偏移超出原生有符号上限时 panic Overflow。
- QuoRem 的商向零截断且 scale=0；余数 scale=max 输入 scale，与被除数同号。Mod 返回相同余数；Rust 名称为 `r#mod`，另有 `modulo` 别名。零除数 panic DivideByZero。
- 已确认的资源行为差异：Go 基线的 QuoRem 在检查零除数前对齐 scale；对 `1 / (0 × 10^-isize::MAX)` 可能先耗尽资源。Rust 在对齐前检查零除数，保持契约中的 DivideByZero 类型化 panic，避免无意义分配；Mod 两个入口同样受益。该差异不改变数值结果、scale 或错误类别，属于不要求匹配的分配行为；极端输入只在 Rust 回归中执行，不用 Go 进程耗尽资源来验证。
- Power 保留输入 scale、最终 HalfEven；0^0=1，零的负幂 panic DivideByZero。去尾零后先处理 ±1；其他幂的绝对指数乘 max(1,scale,系数位长) 超过 i32::MAX 时 panic Overflow，包含 i64::MIN 防溢出路径。
- IntPart 向零取整数；Remainder 返回整数部分和未缩放的带符号分数系数；IsInteger/HasFraction 不经浮点判断。输入均不变，返回大整数独立持有。
