---
locale: ja
source_revision: 668ba6d01bb4e8f8fc531b677bb3554fa942e3fa2858410dfc728efffb55ae7f
translation_provenance: en-US-japanese-reviewed
api_id: "validation"
---
# validation

規則の失敗を順序どおりに収集して翻訳します。

```rust
pub mod validation
```

## 引数、戻り値、通常の使い方

Rust は validation 機能を必要とし、ブラウザービルドでは有効です。このモジュールはフレームワークやグローバルレジストリーを使わず、正確な規則評価と所有メッセージテンプレートをまとめます。ブラウザーでは validation 名前空間を使います。フィールド名は呼び出し側のテキストであり、翻訳はリテラルとして置換します。欠落入力を失敗にする必要がある場合は required を明示してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
