---
locale: ja
source_revision: db21c0833b97369db1efcc9a73b34501c5dcac820064e9658bbb1cb161d8dec2
translation_provenance: en-US-japanese-reviewed
api_id: "validation::default_validation_messages"
---
# validation::default_validation_messages

ロケールのテンプレートの新しいマップを取得します。

```rust
fn default_validation_messages (locale : & str) -> BTreeMap < String , String >
```

## 引数、戻り値、通常の使い方

locale: &str/string。12 種類すべての規則テンプレートを含む `BTreeMap<String,String>`/通常のオブジェクトを返します。不明なロケールは英語を使います。各呼び出しはマップを独立して所有し、編集しても後の呼び出しや他の ValidationMessages には影響しません。再エクスポートは同じ関数の公開モジュールインポートパスです。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-default-validation-messages" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
