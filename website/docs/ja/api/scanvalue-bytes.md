---
locale: ja
source_revision: 69334c1aa4ddf9b63780cdd0f7b3c783c6e1646067bf4562ab026efd47279431
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::Bytes"
---
# ScanValue::Bytes

SQL 入力の Bytes バリアントを選択します。

```rust
Bytes (& 'a [u8])
```

## 引数、戻り値、通常の使い方

バイナリ／BSON の通信データではなく、小数テキストのバイト列を借用します。ブラウザーは scan まで Uint8Array を保持し、それ以前の編集を反映します。scan は入力を変更しません。不正な UTF-8 は小数解析に失敗します。バリアントは scan に借用され、同じブラウザー実行内で再利用できます。偽造した値や別の実行の値は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-bytes" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
