---
locale: zh-Hant
source_revision: 2264e63e21d4ba0bb0732ee23526e08fc191d66bd401bfd2f899bab466b9c36e
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_from_string
---
# Decimal::new_from_string

直接解析精確十進位文字，不經過二進位浮點數。通常用於使用者輸入的小數金額。

```rust
fn new_from_string (text : & str) -> Result < Self , DecimalError >
```

## 參數和回傳值

`text: &str` 對應 JavaScript `string`。Rust 回傳 `Result<Decimal, DecimalError>`；bridge 回傳 Decimal 或拋出保留程式庫分類的錯誤。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-string" locale="zh-Hant" />

## 使用說明

輸入會去掉首尾空白，可包含符號和指數。小數點兩邊都必須有數字，分隔符始終是 `.`，不隨介面語言改變。非零值保留尾零，但 `"0.00"` 會正規化為已初始化的 scale 為零的零。錯誤語法回傳 `InvalidFormat`；指數展開後可能觸及獨立的 Playground 資源限制。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
