---
locale: zh-Hant
source_revision: 9459cb9d77c7f6ad22b96853baef01b1a9461529cecff18a116b5d2dee9166d3
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal
---
# Decimal

儲存精確整數係數及非負的小數位數。

```rust
pub struct Decimal
```

## 常規用法及語義

數值為 coefficient × 10^-scale。scale 表示小數位數，包括保留的末尾零。預設值是未初始化的數值零。建構函數得到已初始化數值；借用方法保留輸入，消耗方法會使瀏覽器包裝物件失效。bridge 使用具名建構函數，new Decimal() 不是公開建構方式。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal" locale="zh-Hant" />

這份共用範例同時在原生 Rust 和瀏覽器中執行。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
