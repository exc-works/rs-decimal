---
locale: ja
source_revision: 377152e0483ddcbebcd02f161fe043c129efdc1f915e3eb70b399c28e3d3906b
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationError"
---
# validation::ValidationError

失敗した検証レコードを確認します。

```rust
pub struct ValidationError
```

## 引数、戻り値、通常の使い方

公開プロパティ field、tag、parameter を持つ失敗レコードです。ValidationRule.validate は失敗時にこれを返し、validate_decimal はリストを返します。JavaScript は翻訳に使える編集可能なフィールドレコードを公開します。DecimalError の種類や例外メッセージではありません。レコードを編集しても規則や小数は変わりません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror" locale="ja" />

## ブラウザーの制約

validation 機能は有効です。各文字列は UTF-8 で 8192 バイト、規則／エラーのリストとメッセージマップは 64 項目、JSON 転送は 65536 バイトまでです。規則の小数引数は[リソースポリシー](../resource-policy)に従います。翻訳前に置換後テキストの保守的な合計上限 65536 バイトを検査し、実際にはちょうど収まる結果も拒否する場合があります。独自のタグとフィールドはデータであり、実行するコードではありません。検証失敗、設定エラー、bridge のリソースエラーは区別します。

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
