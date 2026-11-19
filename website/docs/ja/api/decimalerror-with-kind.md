---
locale: ja
source_revision: ecc694dc9095e4bf91fee90f15b14f69b82899c7811b1a6abfdda76dd5e06c56
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError::with_kind"
---
# DecimalError::with_kind

エラーを消費して種類を追加します。

```rust
fn with_kind (mut self , kind : ErrorKind) -> Self
```

## 引数、戻り値、通常の使い方

kind: ErrorKind。self を消費し、新しい DecimalError を返します。既存の種類とメッセージを保持し、重複は無視します。元のブラウザーハンドルは使えなくなります。不正な JS kind 値はムーブ前に拒否します。ブラウザーの名前付きコンストラクターは、message、matches()、kinds()、with_kind() を持つ解放可能な Rust ベースの値を作成します。送出されたライブラリ例外は category="library" と kinds 配列を持つネイティブ JS Error です。そこでは e.kinds.includes(ErrorKind.Unmarshal) を使います。parameter/resource/moved の bridge エラーは別で、ライブラリの種類は持ちません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-with-kind" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
