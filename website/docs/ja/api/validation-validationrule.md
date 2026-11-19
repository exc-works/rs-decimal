---
locale: ja
source_revision: 4a04c1296f4a9e4d0c1eac8d1707242f3d7b87d7302eac99dc6a590f2c03208e
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationRule"
---
# validation::ValidationRule

正確な小数の規則を評価します。

```rust
pub struct ValidationRule
```

## 引数、戻り値、通常の使い方

parse(tag, parameter) で規則を作成し、matches や validate で再利用します。引数は構築時に 1 回検証します。ブラウザーのハンドルは 1 回の実行に属し、明示的に解放できます。規則は浮動小数点変換なしに正確な Decimal 値を比較します。decimal_required は初期化状態を調べるため、初期化済みのゼロは通過します。eq/ne/gt/gte/lt/lte は未初期化のゼロも数値として比較します。between は両端を含む lower~upper を使い、初期化も要求します。positive/negative/nonzero も初期化を要求します。max_precision は末尾のゼロを含む保存スケールを調べ、required は暗黙に要求しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
