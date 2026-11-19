---
locale: ja
source_revision: 51d8eb06be2cf1b2c76ae713dc34443717bd6adc00c52dec95e805914eed6ec1
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError::matches"
---
# DecimalError::matches

指定した種類を持つか調べます。

```rust
fn matches (& self , kind : ErrorKind) -> bool
```

## 引数、戻り値、通常の使い方

kind: ErrorKind。bool/boolean を返します。借用はエラーを変更しません。1 つのエラーは同時に複数の種類に一致できます。ブラウザーの名前付きコンストラクターは、message、matches()、kinds()、with_kind() を持つ解放可能な Rust ベースの値を作成します。送出されたライブラリ例外は category="library" と kinds 配列を持つネイティブ JS Error です。そこでは e.kinds.includes(ErrorKind.Unmarshal) を使います。parameter/resource/moved の bridge エラーは別で、ライブラリの種類は持ちません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-matches" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
