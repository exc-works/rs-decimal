---
locale: ja
source_revision: 4973633c1cdc3ddfb696d250ca871028b9835fa4187520f393245e4690f6a0c3
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationMessages::with_messages"
---
# validation::ValidationMessages::with_messages

英語の検証テンプレートを上書きします。

```rust
fn with_messages (messages : BTreeMap < String , String >) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

messages: `BTreeMap<String,String>`/文字列値のオブジェクト。`Result<ValidationMessages, DecimalError>`/メッセージハンドルを返します。指定したキーだけを上書きし、他の規則は英語の既定値を保持します。許可されるキーは 12 個の decimal_* 規則です。空のテンプレート、不明なキー、対応しない波括弧、{0}/{1} 以外のプレースホルダーは InvalidArgument です。有効なプレースホルダーの繰り返しは許可します。グローバルなテンプレートや構築済みのテンプレートは変更しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-with-messages" locale="ja" />

## ブラウザーの制約

validation 機能は有効です。各文字列は UTF-8 で 8192 バイト、規則／エラーのリストとメッセージマップは 64 項目、JSON 転送は 65536 バイトまでです。規則の小数引数は[リソースポリシー](../resource-policy)に従います。翻訳前に置換後テキストの保守的な合計上限 65536 バイトを検査し、実際にはちょうど収まる結果も拒否する場合があります。独自のタグとフィールドはデータであり、実行するコードではありません。検証失敗、設定エラー、bridge のリソースエラーは区別します。

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
