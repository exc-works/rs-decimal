---
locale: ja
source_revision: a985525078f122e92ef2d70bf9a962a63b82b868ba67925f976407356bf4ff6b
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError::kinds"
---
# DecimalError::kinds

エラーの全種類を読み取ります。

```rust
fn kinds (& self) -> & [ErrorKind]
```

## 引数、戻り値、通常の使い方

引数なし。&[ErrorKind]/独立したバリアント名の配列を返します。種類は重複を除去し、アルファベット順ではなく Rust の列挙順で並びます。ブラウザーの名前付きコンストラクターは、message、matches()、kinds()、with_kind() を持つ解放可能な Rust ベースの値を作成します。送出されたライブラリ例外は category="library" と kinds 配列を持つネイティブ JS Error です。そこでは e.kinds.includes(ErrorKind.Unmarshal) を使います。parameter/resource/moved の JavaScript エラーは別で、ライブラリの種類は持ちません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-kinds" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
