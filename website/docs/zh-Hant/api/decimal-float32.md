---
locale: zh-Hant
source_revision: 821c4c23615da2c03d94630c3843c11502e27fc2644faac01dc3a81234b52029
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::float32
---
# Decimal::float32

直接轉換為 IEEE binary32 並報告精確性。

```rust
fn float32 (& self) -> (f32 , bool)
```

## 參數、回傳值及常規用法

無參數；Rust 回傳 (f32, bool)，映射為 [number, exact]。正中間值取偶數。旗標表示十進位數被精確表示，而非僅表示轉換成功：0.1 不精確。溢出回傳帶符號無窮和 false；下溢可回傳帶符號零。直接捨入到 binary32，不先捨入到 binary64。範例記錄 IEEE 位模式，避免 JSON 隱藏特殊值。Playground print 也會為 NaN、無窮和負零標注傳輸時的 binary64 位模式。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-float32" locale="zh-Hant" />

## 瀏覽器邊界

整數不會經過有損的 JS number 轉換。轉換元組按 Rust 順序保留值和旗標；旗標為 false 時不能假定結果精確。借用輸入保持不變，建構結果已初始化。[資源限制](../resource-policy) 限制整數輸入、輸出精度及有理數係數增長，與程式庫錯誤分開報告。僅 Rust 匯入語法的範例標記為原生測試。

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
