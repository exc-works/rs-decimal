---
locale: zh-Hant
source_revision: ddfedf2b2a7b0914238752b884265b0c43b55c647cf39e728c8143e706a5b5ba
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::sub_owned
---
# Decimal::sub_owned

消耗左運算元進行精確運算。

```rust
fn sub_owned (self , other : & Self) -> Self
```

## 參數、回傳值及常規用法

other: &Decimal；回傳小數位數為兩個輸入中較大者的已初始化 Decimal。接收者被移動，other 仍可使用。可能重複使用儲存，但增長仍會分配。bridge 在移動前拒絕同一物件同時作為接收者和 other。進入 Rust 後，即使資源錯誤也會消耗接收者。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-sub-owned" locale="zh-Hant" />

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
