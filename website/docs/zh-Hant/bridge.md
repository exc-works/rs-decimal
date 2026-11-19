---
locale: zh-Hant
source_revision: 967fc6e56bfb3ade48f0faf10b0c1e0de7e21a4d9ec093d537fb469385460f6f
translation_provenance: zh-Hans-terminology-reviewed
---
# 在瀏覽器中呼叫 Rust 方法

Playground 使用 JavaScript 呼叫編譯為 Wasm 的 Rust 程式庫。方法名、參數順序、
捨入規則和十進位狀態與 Rust 對應；JavaScript 只負責組織流程，不重新計算
十進位結果。各方法的行為見[方法目錄](./api/)，範例可在 [Playground](./playground) 中執行。

```javascript
const price = Decimal.new_from_string('12.350');
const total = price.mul_exact(Decimal.new(3n));
total.string_with_trailing_zeros(); // '37.050'
```

十進位輸入使用原始字串，i64 參數使用 `bigint`（`3n`，不是 `3`）。
`Decimal.new` 拒絕超出 i64 範圍的值、數字字串和 JS number。
precision 使用 wasm32 有符號整數範圍內的 JS 整數；小數、非有限值和越界值
在 ABI 轉換前被拒絕，避免截斷或回繞。負整數 precision 會傳入 Rust，
由原方法回傳 InvalidPrecision。

## 錯誤與所有權

用例外的 `kinds` 陣列判斷是否發生了 `InvalidPrecision` 等錯誤。參數型別或範圍錯誤時，類別為 `parameter`。

呼叫 `rescale` 後，原值仍可參與其他計算。`into_rescaled` 會消耗原值，即使捨入失敗也是如此。需要保留原值時，請先呼叫 `clone`。

## 可空值與位元組

`NullDecimal.new(decimal)` 消耗參數並設定 `valid=true`，即使參數是
未初始化零。`NullDecimal.default()` 表示缺失。修改 `valid` 只改變有效性，
不會清空內部值。`get_decimal_clone()` 是顯式命名的適配方法，回傳獨立副本；
`set_decimal(value)` 移入新的內部值，並保留 valid。沒有會誤導為即時參照的
`.decimal` getter。

`coefficient()` 回傳不可變的 JS bigint 快照。JavaScript 不復現 Rust 的
借用生命週期和編譯期 trait 檢查。`ByteBuffer.new(Uint8Array)` 複製輸入位元組，
`to_uint8_array()` 回傳副本，不受 Wasm 記憶體增長影響。追加文字方法見[方法目錄](./api/)。

JavaScript 的 `snapshot(value)` 立即捕獲精確狀態，包括無效 NullDecimal 中
保留的內部值。回傳的資料被凍結，後續修改物件不會改變舊輸出。
隔離執行器限制遞迴輸出和執行時間，詳見[資源限制](./resource-policy)。
