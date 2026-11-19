---
locale: ja
source_revision: 52cfb6e6f9d640e445d0e6b774455d4a9cdc44bf1d1bc94d3efd717749548fc4
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError::new"
---
# DecimalError::new

1 つの種類を持つエラーを作成します。

```rust
fn new (kind : ErrorKind , message : impl Into < String >) -> Self
```

## 引数、戻り値、通常の使い方

kind: ErrorKind、message: impl `Into<String>`/string。新しい DecimalError を返します。メッセージは人向けであり、種類への所属を決めません。ブラウザーの名前付きコンストラクターは、message、matches()、kinds()、with_kind() を持つ解放可能な Rust ベースの値を作成します。送出されたライブラリ例外は category="library" と kinds 配列を持つネイティブ JS Error です。そこでは e.kinds.includes(ErrorKind.Unmarshal) を使います。parameter/resource/moved の JavaScript エラーは別で、ライブラリの種類は持ちません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-new" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
