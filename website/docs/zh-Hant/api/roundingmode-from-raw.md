---
locale: zh-Hant
source_revision: e9709f45197c4a553d9eef583ec3ace47a8ab16090d52890478f7fffb689b911
translation_provenance: zh-Hans-terminology-reviewed
api_id: RoundingMode::from_raw
---
# RoundingMode::from_raw

驗證整數捨入模式編碼。

```rust
fn from_raw (value : i32) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

value: i32/整數 number。0–6 回傳對應模式，其他 i32 值回傳 InvalidArgument。瀏覽器小數、非有限或超出 i32 範圍的值會在 Rust 呼叫前被參數檢查拒絕。bridge 呼叫 Rust 驗證器並回傳數字模式。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-from-raw" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
