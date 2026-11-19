---
locale: ja
source_revision: c829b94ceed61e0a65772da94750a53ecb8269151bdb6f1efd6a7c407ad09f12
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

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
