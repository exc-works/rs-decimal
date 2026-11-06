# API 兼容矩阵

固定基线：`a8791208e2d15483cf652cd2814b74b0c1a84187`。PR 03 已实现类型、基础构造、解析和比较；仅标为已验证的条目通过了当前测试，其他入口仍未完成。完整 Go 签名、来源位置和构建条件见 [声明清单](api-inventory.json)。状态、字段和验收规则见 [技术方案](technical-design.md#api-兼容矩阵)。

PR 01 确定交付归属和处理方式；Rust 签名、逐入口行为细节、失败顺序及测试证据在对应实现 PR 中补齐。表内“待设计”不可计为兼容通过。核心入口遵循 [数值契约](../testdata/upstream/docs/numeric-contract.md)，解码入口必须另列 null 与失败状态规则。

| Go API ID | Rust 映射/签名 | 层次 | PR | 处理方式 | 数值/scale/状态 | 失败形态/顺序 | 验证证据 | 状态 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Between` | `between(v: &Decimal, lower: &Decimal, upper: &Decimal) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal` | `Decimal { coefficient: Option<BigInt>, scale: usize }`（私有字段） | 核心 | 03 | 直接移植 | Default 未初始化零；Valid 独立于内部 Decimal | 由类型和构造器保持内部不变量 | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Abs` | `Decimal::abs(&self) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Add` | Decimal::add（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.AddRaw` | Decimal::add_raw（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.AppendText` | Decimal::append_text（签名待设计） | 核心 | 10a | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.AppendTextWithTrailingZeros` | Decimal::append_text_with_trailing_zeros（签名待设计） | 核心 | 10a | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.ApproxRoot` | Decimal::approx_root（签名待设计） | 核心 | 07 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.ApproxRootWithPrec` | Decimal::approx_root_with_prec（签名待设计） | 核心 | 07 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.BigInt` | `Decimal::big_int(&self) -> BigInt` | 核心 | 06 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.BigRat` | Decimal::big_rat（签名待设计） | 核心 | 06 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.BitLen` | Decimal::bit_len（签名待设计） | 核心 | 06 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Ceil` | Decimal::ceil（签名待设计） | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.CeilWithPrec` | Decimal::ceil_with_prec（签名待设计） | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Clone` | `Decimal::clone(&self) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Cmp` | `Decimal::cmp(&self, other: &Self) -> i32` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Equal` | `Decimal::equal(&self, other: &Self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Exp` | Decimal::exp（签名待设计） | 核心 | 09 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.ExpWithPrec` | Decimal::exp_with_prec（签名待设计） | 核心 | 09 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Float32` | Decimal::float32（签名待设计） | 核心 | 06 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Float64` | Decimal::float64（签名待设计） | 核心 | 06 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Floor` | Decimal::floor（签名待设计） | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.FloorWithPrec` | Decimal::floor_with_prec（签名待设计） | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Format` | Decimal::format（签名待设计） | 核心 | 10a | 平台映射：独立 Go 格式入口 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.FormatWithSeparators` | Decimal::format_with_separators（签名待设计） | 核心 | 10a | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.GT` | `Decimal::gt(&self, other: &Self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.GTE` | `Decimal::gte(&self, other: &Self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.HasFraction` | Decimal::has_fraction（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Int64` | Decimal::int64（签名待设计） | 核心 | 06 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.IntPart` | Decimal::int_part（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.IsInteger` | Decimal::is_integer（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
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
| `Decimal.Mod` | Decimal::mod（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Mul` | Decimal::mul（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.MulDown` | Decimal::mul_down（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.MulExact` | Decimal::mul_exact（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.MulWithPrec` | Decimal::mul_with_prec（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.MustNonNegative` | Decimal::must_non_negative（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Neg` | `Decimal::neg(&self) -> Self` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.NotEqual` | `Decimal::not_equal(&self, other: &Self) -> bool` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Power` | Decimal::power（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Precision` | `Decimal::precision(&self) -> isize` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.Quo` | Decimal::quo（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.QuoDown` | Decimal::quo_down（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.QuoRem` | Decimal::quo_rem（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.QuoWithPrec` | Decimal::quo_with_prec（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Remainder` | Decimal::remainder（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Rescale` | Decimal::rescale（签名待设计） | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.RescaleDown` | Decimal::rescale_down（签名待设计） | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Round` | Decimal::round（签名待设计） | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.RoundWithPrec` | Decimal::round_with_prec（签名待设计） | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.SafeAdd` | Decimal::safe_add（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.SafeSub` | Decimal::safe_sub（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Scan` | Decimal::scan（签名待设计） | 协议 | 13d | 平台映射 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Shift` | Decimal::shift（签名待设计） | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Sign` | `Decimal::sign(&self) -> i32` | 核心 | 03 | 直接移植 | [PR 03 行为规则](#pr-03-行为规则) | [PR 03 失败规则](#pr-03-行为规则) | `tests/representation.rs`；`representation-0..299` 差分（声明项由类型/错误测试覆盖） | 已验证 |
| `Decimal.SignificantFigures` | Decimal::significant_figures（签名待设计） | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Sqrt` | Decimal::sqrt（签名待设计） | 核心 | 07 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.SqrtWithPrec` | Decimal::sqrt_with_prec（签名待设计） | 核心 | 07 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.String` | Decimal::string（签名待设计） | 核心 | 10a | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.StringWithTrailingZeros` | Decimal::string_with_trailing_zeros（签名待设计） | 核心 | 10a | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.StripTrailingZeros` | Decimal::strip_trailing_zeros（签名待设计） | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Sub` | Decimal::sub（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.SubRaw` | Decimal::sub_raw（签名待设计） | 核心 | 05 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.Truncate` | Decimal::truncate（签名待设计） | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `Decimal.TruncateWithPrec` | Decimal::truncate_with_prec（签名待设计） | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
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
| `RoundCeiling` | 待设计 | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `RoundDown` | 待设计 | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `RoundHalfDown` | 待设计 | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `RoundHalfEven` | 待设计 | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `RoundHalfUp` | 待设计 | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `RoundUnnecessary` | 待设计 | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `RoundUp` | 待设计 | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
| `RoundingMode` | 待设计 | 核心 | 04 | 直接移植 | 待设计 | 待设计 | 待实现 | 未开始 |
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
