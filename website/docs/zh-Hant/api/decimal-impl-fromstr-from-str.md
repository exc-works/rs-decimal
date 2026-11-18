---
locale: zh-Hant
source_revision: b8a64fe28faf9f699b61b2f86e331fd9c5b8333882ce85a4476ea07f56d57e1d
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::impl FromStr::from_str
---
# Decimal::impl FromStr::from_str

透過 Rust 的 parse 和 FromStr 解析小數文字。

```rust
fn from_str (text : & str) -> Result < Self , Self :: Err >
```

## 常規用法及語義

回傳 Decimal 或 DecimalError。語法及零值標準化與 new_from_string 相同。瀏覽器呼叫該具名方法；Rust trait 語法僅適用於原生 Rust。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr-from-str" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
