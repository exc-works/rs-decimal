---
locale: zh-Hant
source_revision: 16147351e8fe026905ac983886e4a94a6ce341eb3c301ee1901cd0719ea82ee1
translation_provenance: zh-Hans-terminology-reviewed
api_id: RoundingMode::from_raw
---
# RoundingMode::from_raw

驗證整數捨入模式編碼。

```rust
fn from_raw (value : i32) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

value: i32/整數 number。0–6 回傳對應模式，其他 i32 值回傳 InvalidArgument。瀏覽器小數、非有限或超出 i32 範圍的值會在 Rust 呼叫前被參數檢查拒絕。bridge 呼叫 Rust 驗證器並回傳數字模式。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-from-raw" locale="zh-Hant" />

## 瀏覽器邊界

所有整數參數必須位於 wasm32 對應範圍。瀏覽器還施加 [資源限制](../resource-policy)，包括目標小數位數、係數增長及呼叫次數。資源錯誤與數值程式庫錯誤分開報告。借用方法不會消耗輸入；僅 Rust 的 trait 語法明確標注為原生測試。

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
