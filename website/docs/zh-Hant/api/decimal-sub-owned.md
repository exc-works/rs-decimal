---
locale: zh-Hant
source_revision: 347ad719aa4b33bd925c7dfe63b5e557c57b390b4d8d0b57d7aff0d9ca62ac93
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::sub_owned
---
# Decimal::sub_owned

消耗左運算元進行精確運算。

```rust
fn sub_owned (self , other : & Self) -> Self
```

## 參數、回傳值及常規用法

other: &Decimal；回傳小數位數為兩個輸入中較大者的已初始化 Decimal。接收者被移動，other 仍可使用。可能重複使用儲存，但增長仍會分配。JavaScript 在移動前拒絕同一物件同時作為接收者和 other。進入 Rust 後，即使資源錯誤也會消耗接收者。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-sub-owned" locale="zh-Hant" />

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
