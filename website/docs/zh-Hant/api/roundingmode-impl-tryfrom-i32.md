---
locale: zh-Hant
source_revision: e38006128b404f87e5b765e2fa64c4d3f058b23c6a114c6e9a68f4ed1634f91f
translation_provenance: zh-Hans-terminology-reviewed
api_id: "RoundingMode::impl TryFrom < i32 >"
---
# RoundingMode::impl TryFrom < i32 >

透過 TryFrom 驗證原始捨入模式值。

```rust
impl TryFrom < i32 > for RoundingMode
```

## 常規用法與注意事項

value: i32；回傳 `Result<RoundingMode, DecimalError>`。0–6 使用固定映射，其中 5 為 HalfEven。未知值回傳 InvalidArgument，不 panic。關聯 Error 類型為 DecimalError。此實現委託 from_raw；瀏覽器呼叫 RoundingMode.from_raw(number)，嚴格檢查 i32 參數。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-roundingmode-impl-tryfrom-i32" locale="zh-Hant" />

## 執行方式

此條目是 Rust 原生 trait 語法，不提供同名 JavaScript 運算子或 SQLx 物件。 此範例由預設原生文件測試執行。

## 相關方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
