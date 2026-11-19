---
locale: ja
source_revision: e6b8dfbe3d3140a4697ec5a94f4d508f975bc9c0847e91c70ef29e95978ddcfb
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind"
---
# ErrorKind

安定したライブラリエラーの種類を説明します。

```rust
pub enum ErrorKind
```

## 引数、戻り値、通常の使い方

列挙型には 11 種類あり、Rust でコピーと順序比較ができます。ブラウザーの ErrorKind は Rust から読み込んだ凍結済みの名前マップです。個別のエラー処理にはメッセージ比較ではなく、種類への所属を使います。ブラウザーの名前付きコンストラクターは、message、matches()、kinds()、with_kind() を持つ解放可能な Rust ベースの値を作成します。送出されたライブラリ例外は category="library" と kinds 配列を持つネイティブ JS Error です。そこでは e.kinds.includes(ErrorKind.Unmarshal) を使います。parameter/resource/moved の JavaScript エラーは別で、ライブラリの種類は持ちません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
