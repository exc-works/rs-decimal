---
locale: zh-Hant
source_revision: 8e571c100355b6f464b7992d5d9d515f9f9923a4538f1e43b8cc48af4b41a4c7
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::coefficient
---
# Decimal::coefficient

查看精確的未縮放整數係數。

```rust
fn coefficient (& self) -> & BigInt
```

## 參數、回傳值及注意事項

Rust 借用 &BigInt；JavaScript 回傳不可變的 bigint 快照，並非即時參照。係數包含末尾零，不是小數的整數部分。範例把 bigint 轉成文字，以便 JSON 保留全部數字。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-coefficient" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
