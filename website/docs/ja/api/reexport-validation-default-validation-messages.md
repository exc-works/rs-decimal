---
locale: ja
source_revision: 96429a6b772c7e5f91528e442f9be3179ac231b38327e61764d5f24018444428
translation_provenance: en-US-japanese-reviewed
api_id: "reexport::validation::default_validation_messages"
---
# reexport::validation::default_validation_messages

ロケールのテンプレートの新しいマップを取得します。

```rust
pub use validation::messages::default_validation_messages as default_validation_messages
```

## 引数、戻り値、通常の使い方

locale: &str/string。12 種類すべての規則テンプレートを含む `BTreeMap<String,String>`/通常のオブジェクトを返します。不明なロケールは英語を使います。各呼び出しはマップを独立して所有し、編集しても後の呼び出しや他の ValidationMessages には影響しません。再エクスポートは同じ関数の公開モジュールインポートパスです。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-reexport-validation-default-validation-messages" locale="ja" />

## ブラウザーの制約

validation 機能は有効です。各文字列は UTF-8 で 8192 バイト、規則／エラーのリストとメッセージマップは 64 項目、JSON 転送は 65536 バイトまでです。規則の小数引数は[リソースポリシー](../resource-policy)に従います。翻訳前に置換後テキストの保守的な合計上限 65536 バイトを検査し、実際にはちょうど収まる結果も拒否する場合があります。独自のタグとフィールドはデータであり、実行するコードではありません。検証失敗、設定エラー、bridge のリソースエラーは区別します。

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
