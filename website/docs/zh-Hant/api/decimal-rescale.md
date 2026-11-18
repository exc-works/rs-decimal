---
locale: zh-Hant
source_revision: 0deb49519f3f009c2dba973109de20fc0fcdfe90b9617749c506123c4c387997
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::rescale
---
# Decimal::rescale

使用明確的捨入模式改變小數位數。

```rust
fn rescale (& self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

prec 表示小數位數，不是有效位數位數；mode 是七種 RoundingMode 值之一。提高精度追加零；降低精度時只對被丟棄的非零數字捨入。負精度回傳 InvalidPrecision；Unnecessary 遇到需要捨入時回傳 RoundUnnecessary。結果會將預設零初始化；借用的輸入保持不變。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale" locale="zh-Hant" />

## 瀏覽器邊界

所有整數參數必須位於 wasm32 對應範圍。瀏覽器還施加 [資源限制](../resource-policy)，包括目標小數位數、係數增長及呼叫次數。資源錯誤與數值程式庫錯誤分開報告。借用方法不會消耗輸入；僅 Rust 的 trait 語法明確標注為原生測試。

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
