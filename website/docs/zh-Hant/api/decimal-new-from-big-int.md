---
locale: zh-Hant
source_revision: cc346dbc73e3d203c64f61c0e3e68ec08682dec8479c74ee4ea48b4441aef5c2
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_from_big_int
---
# Decimal::new_from_big_int

把任意精度整數複製為已初始化、scale 為零的 Decimal，不丟失數字。

```rust
fn new_from_big_int (value : & BigInt) -> Self
```

## 參數和回傳值

Rust 借用 `&BigInt`；JavaScript 傳入不可變的 `bigint`。回傳值的係數獨立於來源。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-big-int" locale="zh-Hant" />

## 使用說明

不要先用 JavaScript number 丟失精度後再轉 bigint；應直接使用以 `n` 結尾的整數字面量或精確整數文字。Playground 最多接受 4096 位輸入數字；原生程式庫不受此 Playground 策略限制。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
