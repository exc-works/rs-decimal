---
locale: ja
source_revision: 3bfbb5926961d9d892e3da2738bc0fc6133b23f056d8610803f8a02d24a533d4
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind"
---
# ErrorKind

安定したライブラリエラーの種類を説明します。

```rust
pub enum ErrorKind
```

## 引数、戻り値、通常の使い方

列挙型には 11 種類あり、Rust でコピーと順序比較ができます。ブラウザーの ErrorKind は Rust から読み込んだ凍結済みの名前マップです。個別のエラー処理にはメッセージ比較ではなく、種類への所属を使います。ブラウザーの名前付きコンストラクターは、message、matches()、kinds()、with_kind() を持つ解放可能な Rust ベースの値を作成します。送出されたライブラリ例外は category="library" と kinds 配列を持つネイティブ JS Error です。そこでは e.kinds.includes(ErrorKind.Unmarshal) を使います。parameter/resource/moved の bridge エラーは別で、ライブラリの種類は持ちません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
