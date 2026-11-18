---
locale: zh-Hant
source_revision: 38cd98cd0eb4f9b158b56009f272819540665b187722c4eef35585af26929132
translation_provenance: zh-Hans-terminology-reviewed
api_id: COMPATIBILITY_REVISION
---
# COMPATIBILITY_REVISION

查看相容性檢查對應的上游 Go 版本。

```rust
const COMPATIBILITY_REVISION: & str = "a8791208e2d15483cf652cd2814b74b0c1a84187"
```

## 常規用法及語義

這個不可變的 &str 是原始碼版本，不是目前 Rust 或網頁建置提交。瀏覽器從編譯後的 Rust 讀取同一常數，不應將它當作套件版本或部署識別碼。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-compatibility-revision" locale="zh-Hant" />

這份共用範例同時在原生 Rust 和瀏覽器中執行。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
