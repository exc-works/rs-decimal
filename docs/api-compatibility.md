# API 兼容矩阵

固定基线：`a8791208e2d15483cf652cd2814b74b0c1a84187`。PR 03—09 已实现类型、算术、转换和认证数学函数；仅标为已验证的条目通过了当前测试，其他入口仍未完成。完整 Go 签名、来源位置和构建条件见 [声明清单](api-inventory.json)。状态、字段和验收规则见 [技术方案](technical-design.md#api-兼容矩阵)。

PR 01 确定交付归属和处理方式；Rust 签名、逐入口行为细节、失败顺序及测试证据在对应实现 PR 中补齐。表内“待设计”不可计为兼容通过。核心入口遵循 [数值契约](../testdata/upstream/docs/numeric-contract.md)，解码入口必须另列 null 与失败状态规则。

| Go API ID | Rust 映射/签名 | 层次 | PR | 处理方式 | 数值/scale/状态 | 失败形态/顺序 | 验证证据 | 状态 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Between` | `between(v: &Decimal, lower: &Decimal, upper: &Decimal) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal` | `Decimal { coefficient: Option<BigInt>, scale: usize }`（私有字段） | 核心 | 03 | 直接移植 | Default 未初始化零；Valid 独立于内部 Decimal | 由类型和构造器保持内部不变量 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Abs` | `Decimal::abs(&self) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Add` | `Decimal::add(&self, other: &Self) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.AddRaw` | `Decimal::add_raw(&self, value: i64) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.AppendText` | `Decimal::append_text(&self, dst: &mut Vec<u8>)` | 核心 | 10a | 直接移植 | [PR 10a 行为规则](#pr-10a-行为规则) | [PR 10a 行为规则](#pr-10a-行为规则) | `tests/formatting.rs`；7137 个 Go/Rust 格式差分 | 已验证 |
| `Decimal.AppendTextWithTrailingZeros` | `Decimal::append_text_with_trailing_zeros(&self, dst: &mut Vec<u8>)` | 核心 | 10a | 直接移植 | [PR 10a 行为规则](#pr-10a-行为规则) | [PR 10a 行为规则](#pr-10a-行为规则) | `tests/formatting.rs`；7137 个 Go/Rust 格式差分 | 已验证 |
| `Decimal.ApproxRoot` | `Decimal::approx_root(&self, degree: i64) -> Result<Self, DecimalError>` | 核心 | 07 | 直接移植 | [PR 07 行为规则](#pr-07-行为规则) | [PR 07 行为规则](#pr-07-行为规则) | `tests/roots.rs`；173 个差分，150 个根证书，18 个固定 must-succeed | 已验证 |
| `Decimal.ApproxRootWithPrec` | `Decimal::approx_root_with_prec(&self, degree: i64, prec: isize) -> Result<Self, DecimalError>` | 核心 | 07 | 直接移植 | [PR 07 行为规则](#pr-07-行为规则) | [PR 07 行为规则](#pr-07-行为规则) | `tests/roots.rs`；173 个差分，150 个根证书，18 个固定 must-succeed | 已验证 |
| `Decimal.BigInt` | `Decimal::big_int(&self) -> BigInt` | 核心 | 06 | 直接移植 | [PR 06 行为规则](#pr-06-行为规则) | [PR 06 失败规则](#pr-06-行为规则) | `tests/conversion.rs`；`conversion-0..6144` 差分；2046 个浮点中点证书 | 已验证 |
| `Decimal.BigRat` | `Decimal::big_rat(&self) -> BigRational` | 核心 | 06 | 直接移植 | [PR 06 行为规则](#pr-06-行为规则) | [PR 06 失败规则](#pr-06-行为规则) | `tests/conversion.rs`；`conversion-0..6144` 差分；2046 个浮点中点证书 | 已验证 |
| `Decimal.BitLen` | `Decimal::bit_len(&self) -> usize` | 核心 | 06 | 直接移植 | [PR 06 行为规则](#pr-06-行为规则) | [PR 06 失败规则](#pr-06-行为规则) | `tests/conversion.rs`；`conversion-0..6144` 差分；2046 个浮点中点证书 | 已验证 |
| `Decimal.Ceil` | `Decimal::ceil(&self) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.CeilWithPrec` | `Decimal::ceil_with_prec(&self, prec: isize) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.Clone` | `Decimal::clone(&self) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Cmp` | `Decimal::cmp(&self, other: &Self) -> i32` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Equal` | `Decimal::equal(&self, other: &Self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Exp` | `Decimal::exp(&self) -> Result<Self, DecimalError>` | 核心 | 09 | 直接移植 | [PR 09 行为规则](#pr-09-行为规则) | [PR 09 行为规则](#pr-09-行为规则) | `tests/transcendentals.rs`；348 个差分，320 个区间证书；全量 45 个固定 must-succeed | 已验证 |
| `Decimal.ExpWithPrec` | `Decimal::exp_with_prec(&self, prec: isize) -> Result<Self, DecimalError>` | 核心 | 09 | 直接移植 | [PR 09 行为规则](#pr-09-行为规则) | [PR 09 行为规则](#pr-09-行为规则) | `tests/transcendentals.rs`；348 个差分，320 个区间证书；全量 45 个固定 must-succeed | 已验证 |
| `Decimal.Float32` | `Decimal::float32(&self) -> (f32, bool)` | 核心 | 06 | 直接移植 | [PR 06 行为规则](#pr-06-行为规则) | [PR 06 失败规则](#pr-06-行为规则) | `tests/conversion.rs`；`conversion-0..6144` 差分；2046 个浮点中点证书 | 已验证 |
| `Decimal.Float64` | `Decimal::float64(&self) -> (f64, bool)` | 核心 | 06 | 直接移植 | [PR 06 行为规则](#pr-06-行为规则) | [PR 06 失败规则](#pr-06-行为规则) | `tests/conversion.rs`；`conversion-0..6144` 差分；2046 个浮点中点证书 | 已验证 |
| `Decimal.Floor` | `Decimal::floor(&self) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.FloorWithPrec` | `Decimal::floor_with_prec(&self, prec: isize) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.Format` | `Decimal::format_go(&self, f: GoFormat) -> String` | 核心 | 10a | 平台映射：独立 Go 格式入口 | [PR 10a 行为规则](#pr-10a-行为规则) | [PR 10a 行为规则](#pr-10a-行为规则) | `tests/formatting.rs`；7137 个 Go/Rust 格式差分 | 已验证 |
| `Decimal.FormatWithSeparators` | `Decimal::format_with_separators(&self, thousands: char, decimal: char) -> String` | 核心 | 10a | 直接移植 | [PR 10a 行为规则](#pr-10a-行为规则) | [PR 10a 行为规则](#pr-10a-行为规则) | `tests/formatting.rs`；7137 个 Go/Rust 格式差分 | 已验证 |
| `Decimal.GT` | `Decimal::gt(&self, other: &Self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.GTE` | `Decimal::gte(&self, other: &Self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.HasFraction` | `Decimal::has_fraction(&self) -> bool` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.Int64` | `Decimal::int64(&self) -> (i64, bool)` | 核心 | 06 | 直接移植 | [PR 06 行为规则](#pr-06-行为规则) | [PR 06 失败规则](#pr-06-行为规则) | `tests/conversion.rs`；`conversion-0..6144` 差分；2046 个浮点中点证书 | 已验证 |
| `Decimal.IntPart` | `Decimal::int_part(&self) -> BigInt` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.IsInteger` | `Decimal::is_integer(&self) -> bool` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.IsNegative` | `Decimal::is_negative(&self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.IsNil` | `Decimal::is_nil(&self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.IsNotZero` | `Decimal::is_not_zero(&self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.IsPositive` | `Decimal::is_positive(&self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.IsZero` | `Decimal::is_zero(&self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.LT` | `Decimal::lt(&self, other: &Self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.LTE` | `Decimal::lte(&self, other: &Self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Ln` | `Decimal::ln(&self) -> Result<Self, DecimalError>` | 核心 | 09 | 直接移植 | [PR 09 行为规则](#pr-09-行为规则) | [PR 09 行为规则](#pr-09-行为规则) | `tests/transcendentals.rs`；348 个差分，320 个区间证书；全量 45 个固定 must-succeed | 已验证 |
| `Decimal.LnWithPrec` | `Decimal::ln_with_prec(&self, prec: isize) -> Result<Self, DecimalError>` | 核心 | 09 | 直接移植 | [PR 09 行为规则](#pr-09-行为规则) | [PR 09 行为规则](#pr-09-行为规则) | `tests/transcendentals.rs`；348 个差分，320 个区间证书；全量 45 个固定 must-succeed | 已验证 |
| `Decimal.Log10` | `Decimal::log10(&self) -> Result<Self, DecimalError>` | 核心 | 09 | 直接移植 | [PR 09 行为规则](#pr-09-行为规则) | [PR 09 行为规则](#pr-09-行为规则) | `tests/transcendentals.rs`；348 个差分，320 个区间证书；全量 45 个固定 must-succeed | 已验证 |
| `Decimal.Log10WithPrec` | `Decimal::log10_with_prec(&self, prec: isize) -> Result<Self, DecimalError>` | 核心 | 09 | 直接移植 | [PR 09 行为规则](#pr-09-行为规则) | [PR 09 行为规则](#pr-09-行为规则) | `tests/transcendentals.rs`；348 个差分，320 个区间证书；全量 45 个固定 must-succeed | 已验证 |
| `Decimal.Log2` | `Decimal::log2(&self) -> Self` | 核心 | 09 | 直接移植 | [PR 09 行为规则](#pr-09-行为规则) | [PR 09 行为规则](#pr-09-行为规则) | `tests/transcendentals.rs`；348 个差分，320 个区间证书；全量 45 个固定 must-succeed | 已验证 |
| `Decimal.Log2WithPrec` | `Decimal::log2_with_prec(&self, prec: isize) -> Result<Self, DecimalError>` | 核心 | 09 | 直接移植 | [PR 09 行为规则](#pr-09-行为规则) | [PR 09 行为规则](#pr-09-行为规则) | `tests/transcendentals.rs`；348 个差分，320 个区间证书；全量 45 个固定 must-succeed | 已验证 |
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
| `Decimal.Sqrt` | `Decimal::sqrt(&self) -> Result<Self, DecimalError>` | 核心 | 07 | 直接移植 | [PR 07 行为规则](#pr-07-行为规则) | [PR 07 行为规则](#pr-07-行为规则) | `tests/roots.rs`；173 个差分，150 个根证书，18 个固定 must-succeed | 已验证 |
| `Decimal.SqrtWithPrec` | `Decimal::sqrt_with_prec(&self, prec: isize) -> Result<Self, DecimalError>` | 核心 | 07 | 直接移植 | [PR 07 行为规则](#pr-07-行为规则) | [PR 07 行为规则](#pr-07-行为规则) | `tests/roots.rs`；173 个差分，150 个根证书，18 个固定 must-succeed | 已验证 |
| `Decimal.String` | `Decimal::string(&self) -> String` | 核心 | 10a | 直接移植 | [PR 10a 行为规则](#pr-10a-行为规则) | [PR 10a 行为规则](#pr-10a-行为规则) | `tests/formatting.rs`；7137 个 Go/Rust 格式差分 | 已验证 |
| `Decimal.StringWithTrailingZeros` | `Decimal::string_with_trailing_zeros(&self) -> String` | 核心 | 10a | 直接移植 | [PR 10a 行为规则](#pr-10a-行为规则) | [PR 10a 行为规则](#pr-10a-行为规则) | `tests/formatting.rs`；7137 个 Go/Rust 格式差分 | 已验证 |
| `Decimal.StripTrailingZeros` | `Decimal::strip_trailing_zeros(&self) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.Sub` | `Decimal::sub(&self, other: &Self) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.SubRaw` | `Decimal::sub_raw(&self, value: i64) -> Self` | 核心 | 05 | 直接移植 | [PR 05 行为规则](#pr-05-行为规则) | [PR 05 失败规则](#pr-05-行为规则) | `tests/arithmetic.rs`；`arithmetic-0..3378` 差分；2853 个独立 oracle 结果 | 已验证 |
| `Decimal.Truncate` | `Decimal::truncate(&self) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.TruncateWithPrec` | `Decimal::truncate_with_prec(&self, prec: isize) -> Self` | 核心 | 04 | 直接移植 | [PR 04 行为规则](#pr-04-行为规则) | [PR 04 失败规则](#pr-04-行为规则) | `tests/rounding.rs`；`rounding-0..555` 差分；rescale 独立 oracle | 已验证 |
| `Decimal.Uint64` | `Decimal::uint64(&self) -> (u64, bool)` | 核心 | 06 | 直接移植 | [PR 06 行为规则](#pr-06-行为规则) | [PR 06 失败规则](#pr-06-行为规则) | `tests/conversion.rs`；`conversion-0..6144` 差分；2046 个浮点中点证书 | 已验证 |
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
| `MostSignificantBit` | `most_significant_bit(value: &BigInt) -> usize` | 核心 | 06 | 直接移植 | [PR 06 行为规则](#pr-06-行为规则) | [PR 06 失败规则](#pr-06-行为规则) | `tests/conversion.rs`；`conversion-0..6144` 差分；2046 个浮点中点证书 | 已验证 |
| `MustFromString` | `Decimal::must_from_string(text: &str) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `New` | `Decimal::new(value: i64) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NewFromBigInt` | `Decimal::new_from_big_int(value: &BigInt) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NewFromBigIntWithPrec` | `Decimal::new_from_big_int_with_prec(value: &BigInt, prec: isize) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NewFromBigRat` | `Decimal::new_from_big_rat(value: &BigRational) -> Result<Self, DecimalError>` | 核心 | 06 | 直接移植 | [PR 06 行为规则](#pr-06-行为规则) | [PR 06 失败规则](#pr-06-行为规则) | `tests/conversion.rs`；`conversion-0..6144` 差分；2046 个浮点中点证书 | 已验证 |
| `NewFromBigRatWithPrec` | `Decimal::new_from_big_rat_with_prec( value: &BigRational, prec: isize, mode: RoundingMode, ) -> Result<Self, DecimalError>` | 核心 | 06 | 直接移植 | [PR 06 行为规则](#pr-06-行为规则) | [PR 06 失败规则](#pr-06-行为规则) | `tests/conversion.rs`；`conversion-0..6144` 差分；2046 个浮点中点证书 | 已验证 |
| `NewFromDecimal` | `Decimal::new_from_decimal(value: &Self) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `NewFromFloat32` | `Decimal::new_from_float32(value: f32) -> Self` | 核心 | 06 | 直接移植 | [PR 06 行为规则](#pr-06-行为规则) | [PR 06 失败规则](#pr-06-行为规则) | `tests/conversion.rs`；`conversion-0..6144` 差分；2046 个浮点中点证书 | 已验证 |
| `NewFromFloat64` | `Decimal::new_from_float64(value: f64) -> Self` | 核心 | 06 | 直接移植 | [PR 06 行为规则](#pr-06-行为规则) | [PR 06 失败规则](#pr-06-行为规则) | `tests/conversion.rs`；`conversion-0..6144` 差分；2046 个浮点中点证书 | 已验证 |
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
- string/Display 的完整格式化兼容已在 PR 10a 验证。

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

## PR 06 行为规则

- NewFromFloat32/64 使用固定 Ryu 版本产生对应原位宽的最短往返十进制，去掉格式器额外的 .0 后精确解析。负零归一化为初始化零；非有限输入 panic InvalidFormat。随机位模式与 Go 的最短文本选择/scale 逐项比较。
- Float32/64 从精确整数比直接定位二进制网格并 ties-to-even，f32 不经过 f64；保留次正规数与带符号下溢零。溢出返回同号 infinity、exact=false，exact=true 只表示数学上精确相等。
- NewFromBigRat 只接受分母约分后仅含 2/5 因子的有理数，否则返回 InvalidFormat；输出最小终止 scale。WithPrec 先拒绝负 precision，再一次舍入到目标 scale，Unnecessary 非精确时 panic RoundUnnecessary。Rust 的有效引用排除了 Go 的 nil rational 参数，该类型映射不提供空指针入口。
- BigInt/BigRat 返回独立副本。Int64/Uint64 仅接受范围内精确整数，否则返回 (0,false)，不以截断值冒充成功。BitLen 按系数绝对值；MostSignificantBit 的零结果为 0、负输入断言 panic。
- 除 Go 差分外，独立 Python Fraction 验证浮点结果所在的精确中点舍入单元、边界奇偶、符号、overflow/underflow 和 exact 标志；该验证器不调用生产整数舍入。

## PR 07 行为规则

Sqrt/ApproxRoot 默认输出 scale 为 max(input scale, 20)，WithPrec 保留指定 scale，使用精确 HalfEven 中点判断。所有成功结果初始化且不改变输入。
负精度触发 InvalidPrecision panic；非正 degree 返回 InvalidRoot，负偶次根返回 NegativeRoot。degree=1、零和一的快路径先于一般预算；其余输入/输出 scale、degree 及 degree*max(1,p) 受 131072 上限约束，Newton 最多 300 次，耗尽返回 NoConvergence。负奇次根保持符号。
独立 oracle 使用固定 Go 测试中的舍入单元整数不等式，不调用生产根算法；系数、scale、初始化及编码变异必须拒绝。

## PR 08 内部区间验证

区间端点为独立 BigInt，以当前正二进制 scale 归一化。分数和乘除向外舍入，除法接受正分母区间并按分子符号选择极值角点。整数乘法交换负倍率下的端点。仅当完整区间两端 HalfEven 舍入一致时认证成功。
Ziv 从 64 guard bits 开始倍增至 8192，总工作精度仍受 131072 十进制位预算；闭包错误立即返回，预算耗尽为 NoConvergence，输出 scale 不随重试变化。本层保持私有 API；临时 dead_code 标注已在 PR 09 接入使用时移除。
`src/interval.rs` 的精确有理数包含测试覆盖负数、跨零、一般及二进制 scale、乘除和输入不变；`src/transcendental.rs` 覆盖强制重试、8 次耗尽、预算提前拒绝及错误传播。

## PR 09 行为规则

Ln/Log2/Log10/Exp 默认 scale 为 max(input scale,20)，显式精度保留目标 scale；所有成功结果初始化、HalfEven 舍入且输入不变。Log2 默认入口在失败时使用 DecimalError 类型化 panic，Log2WithPrec 与其余入口返回 Result。负精度始终首先触发 InvalidPrecision panic；非正对数返回 InvalidLog。
输入和输出 scale 超过 131072 返回 NoConvergence，预算检查先于 ln(1)/exp(0) 快路径；边界内的精确恒等式、二/十的整数次幂对数仍可成功。Exp 的整数位预算、负大数归零证明和区间重建保留基线顺序。区间端点同舍入才返回结果。
独立 oracle 抽取测试中的精确有理级数及显式余项界，与生产固定点实现分离；变异测试拒绝系数、scale、状态及编码变化。45 个固定承诺成功用例全量匹配输出并独立认证，包括 131072 位精度边界恒等式，不以 NoConvergence 代替成功。

## PR 10a 行为规则

String/Display 输出紧凑固定点十进制；StringWithTrailingZeros 保留 scale，零没有负号。两个 AppendText 入口接受 &mut Vec<u8>，保留原缓冲区前缀并追加 ASCII；Go 返回的恒为 nil 的 error 在 Rust 省略。不承诺与 Go 相同的分配次数。
GoFormat 显式描述单个 verb、width、precision 与五个 flags；format_go 处理 v/s/q/d/f/F/e/E/g/G/b，未知 verb 和非整数 d 返回 Go 错误标记，不是 printf 格式字符串解析器。数字符号、零填充及 d 显式精度的优先级保持基线；g 在舍入后选择记数法。科学格式从系数字符串计算指数，不展开大 scale 零。
FormatWithSeparators 使用 char，NUL thousands 禁止分组，NUL decimal 回退为点。Rust Display 始终返回紧凑文本；Go 专有宽度/精度/flags 使用 format_go，不声称 Rust 的格式语法等同 fmt。
