---
locale: ja
source_revision: 7805847022fc73cbb47e3960021e981cfb8ac4ab47a7b0a542ffa809d986fb9d
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::must_from_string
---
# Decimal::must_from_string

不正な入力をプログラムの誤りと扱う場合に、信頼できる小数テキストを解析します。

```rust
fn must_from_string (text : & str) -> Self
```

## 引数、戻り値、使用上の注意

text: &str/string。Decimal を返します。有効な入力の状態は new_from_string と同じです。不正な構文では Rust は DecimalError でパニックし、コンパイルされた Wasm では回復可能なライブラリエラーではなくトラップになります。ユーザー入力には new_from_string を推奨します。捕捉されないトラップは現在の実行を終了します。Rust を再び使う前に新しい実行を開始してください。引数とリソースの検査は引き続き Rust の呼び出し前に入力を拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-decimal-must-from-string" locale="ja" />

## 関連メソッド

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
