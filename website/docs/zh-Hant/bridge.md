---
locale: zh-Hant
source_revision: bc4fe0d7f63ebbaad6f009ea77ae93eee10d61514aa0fcf8907740ea4397dc70
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

Rust 回傳錯誤時拋出 Error，包含 `name: 'DecimalError'`、
`category: 'library'`、原始 `message` 和完整的 `kinds` 字串陣列。
不要只讀取一個類別：解碼錯誤可能同時屬於多個類別。
JS 類型或範圍錯誤使用 `BridgeError` 和 `category: 'parameter'`，
不偽裝成 Rust 算術錯誤。數值形式的無效捨入模式由 Rust 檢查。

借用方法呼叫後物件仍可使用。`into_rescaled` 會消耗接收者，即使 Rust
回傳錯誤也一樣；再次使用已消耗或釋放的物件會得到 `moved` BridgeError。
不合法的 JS 參數類型在呼叫前被拒絕，因此不會消耗接收者。
`clone()` 保留表示和初始化狀態。`free()` 釋放仍存活的控制代碼，已消耗後呼叫
也安全。執行器完成後將銷毀整個 Wasm 實例，不依賴垃圾回收終結器釋放資源。
不同 bridge 實例的物件不能混用。

## 可空值與位元組

`NullDecimal.new(decimal)` 消耗參數並設定 `valid=true`，即使參數是
未初始化零。`NullDecimal.default()` 表示缺失。修改 `valid` 只改變有效性，
不會清空內部值。`get_decimal_clone()` 是顯式命名的適配方法，回傳獨立副本；
`set_decimal(value)` 移入新的內部值，並保留 valid。沒有會誤導為即時參照的
`.decimal` getter。

`coefficient()` 回傳不可變的 JS bigint 快照。JavaScript 不復現 Rust 的
借用生命週期和編譯期 trait 檢查。`ByteBuffer.new(Uint8Array)` 複製輸入位元組，
`to_uint8_array()` 回傳副本，不受 Wasm 記憶體增長影響。追加文字方法見[方法目錄](./api/)。

bridge 的 `snapshot(value)` 立即捕獲精確狀態，包括無效 NullDecimal 中
保留的內部值。回傳的資料被凍結，後續修改物件不會改變舊輸出。
隔離執行器限制遞迴輸出和執行時間，詳見[資源限制](./resource-policy)。
