---
locale: zh-Hant
source_revision: 50a6a7b47a0234b339cf6d517b9928ec7d1775457e7eacbe40da86286996212a
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::unmarshal_bson_value"
---
# Decimal::unmarshal_bson_value

將 BSON 純量解碼到現有值。

```rust
fn unmarshal_bson_value (& mut self , kind : u8 , data : & [u8]) -> Result < () , DecimalError >
```

## 參數、回傳值及常規用法

kind: u8/number，data: &[u8]/Uint8Array；修改接收者，回傳 `Result<(), DecimalError>`/undefined。Rust 需要 bson。支援 double（1）、string（2）、null（10）、int32（16）、int64（18）和 Decimal128（19）；數值負載使用 BSON 小端位元組。int64 應直接傳原始位元組，避免 JS number 丟失精度。BSON null 將接收者重置為未初始化零，這與保留原值的 unmarshal_json(null) 不同。nullable 成功解碼非空值後設定 valid=true。不支援的類型、截斷資料和非有限數值均失敗且不改變狀態。相容細節：字串解碼跳過聲明的最後一個位元組而不驗證 NUL；超出所需負載的位元組被忽略。Decimal128 由 Rust 核心解碼並保留支援的小數精度；marshal_bson_value 輸出仍是字串。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-bson-value" locale="zh-Hant" />

## 相關方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
