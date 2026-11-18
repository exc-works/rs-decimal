---
locale: zh-Hans
source_revision: b0ab4b73c5319adf6d0f52ca8ad72391d3a1bf1d0a68f2ceac7763706868b54b
---
# 快速开始

输入需要保持精确时，请从字符串构造十进制数。`mul_exact` 将输入的小数位数相加；`string()` 省略尾零，`string_with_trailing_zeros()` 保留尾零。

<<< ../../../examples/web_getting_started.rs

precision 参数通常表示小数位数，不是有效数字位数。`HalfEven` 在中点时取保留末位为偶数的结果。`Down` 朝零截断，不是朝负无穷舍入。

`Decimal::default()` 是未初始化的数值零。初始化零与它数值相等，但状态不同。`NullDecimal.valid` 独立决定值是否存在。

普通可失败方法返回 `Result`。请检查错误类别；只有显式命名的 `must_*` 方法会因预期输入错误而 panic。浏览器将通过 JavaScript bridge 调用相同的 Rust 方法，并说明类型与所有权映射。
