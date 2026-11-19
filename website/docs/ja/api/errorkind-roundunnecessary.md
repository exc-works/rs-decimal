---
locale: ja
source_revision: dc7af33554badb1c95cd11064df3b51faeaca3112afc1b82cf2efcaf4ff1844d
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::RoundUnnecessary"
---
# ErrorKind::RoundUnnecessary

RoundUnnecessary の失敗を識別します。

```rust
RoundUnnecessary
```

## 引数、戻り値、通常の使い方

正確な丸めを要求しましたが、ゼロ以外の桁を捨てる必要があります。ErrorKind はメッセージ文字列ではなく、安定した分類です。ブラウザーの列挙値は Rust のバリアント名です。構築したエラーでは matches() を使い、捕捉した bridge 例外では kinds 配列を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-roundunnecessary" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
