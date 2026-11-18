---
locale: zh-Hant
source_revision: 91bb241ee917c58402441fd00f901b8aa75a693fa5e9137596c74ca66e946c7e
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::int64
---
# Decimal::int64

轉換為固定位寬整數並回傳成功旗標。

```rust
fn int64 (& self) -> (i64 , bool)
```

## 參數、回傳值及常規用法

無參數；回傳 (i64, bool)，映射為 [bigint, boolean]。範圍內的整數回傳對應整數和 true。小數或溢出回傳 (0, false)，不會截斷或拋出錯誤。不能僅憑 scale 判斷是否為整數：12.00 轉換成功。應檢查旗標，區分失敗與有效零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-int64" locale="zh-Hant" />

## 瀏覽器邊界

整數不會經過有損的 JS number 轉換。轉換元組按 Rust 順序保留值和旗標；旗標為 false 時不能假定結果精確。借用輸入保持不變，建構結果已初始化。[資源限制](../resource-policy) 限制整數輸入、輸出精度及有理數係數增長，與程式庫錯誤分開報告。僅 Rust 匯入語法的範例標記為原生測試。

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
