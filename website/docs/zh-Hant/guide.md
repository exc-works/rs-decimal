---
locale: zh-Hant
source_revision: b0ab4b73c5319adf6d0f52ca8ad72391d3a1bf1d0a68f2ceac7763706868b54b
---
# 快速開始

輸入需要保持精確時，請從字串建立十進位數。`mul_exact` 將輸入的小數位數相加；`string()` 省略尾端零，`string_with_trailing_zeros()` 保留尾端零。

<<< ../../../examples/web_getting_started.rs

precision 參數通常表示小數位數，不是有效位數。`HalfEven` 在中點時選擇保留末位為偶數的結果。`Down` 朝零截斷，不是朝負無窮大捨入。

`Decimal::default()` 是未初始化的數值零。初始化零與它數值相等，但狀態不同。`NullDecimal.valid` 獨立決定值是否存在。

一般可失敗方法回傳 `Result`。請檢查錯誤類別；只有明確命名的 `must_*` 方法會因預期輸入錯誤而 panic。瀏覽器將透過 JavaScript bridge 呼叫相同的 Rust 方法，並說明型別與所有權對應。
