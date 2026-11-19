---
locale: ja
source_revision: 189b6d1ee3eb48467bf982330ec482f99ea7cd30e80f4f3c225f2808c0e03cc3
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::RoundUnnecessary"
---
# ErrorKind::RoundUnnecessary

RoundUnnecessary の失敗を識別します。

```rust
RoundUnnecessary
```

## 引数、戻り値、通常の使い方

正確な丸めを要求しましたが、ゼロ以外の桁を捨てる必要があります。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-roundunnecessary" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
