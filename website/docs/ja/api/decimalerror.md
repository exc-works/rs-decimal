---
locale: ja
source_revision: 8805baa34ac15d1bc6208bedc88ae9d1d05c8235150f9f2331a9105561b652f3
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError"
---
# DecimalError

メッセージと 1 つ以上のライブラリエラー種別を保持します。

```rust
pub struct DecimalError
```

## 引数、戻り値、通常の使い方

DecimalError は人が読めるメッセージと、重複を除去して順序付けた種類を保存します。with_kind は値を消費し、matches と kinds は借用します。Rust の Error と Display を実装します。ブラウザーの名前付きコンストラクターは、message、matches()、kinds()、with_kind() を持つ解放可能な Rust ベースの値を作成します。送出されたライブラリ例外は category="library" と kinds 配列を持つネイティブ JS Error です。そこでは e.kinds.includes(ErrorKind.Unmarshal) を使います。parameter/resource/moved の bridge エラーは別で、ライブラリの種類は持ちません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
