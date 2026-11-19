---
locale: ja
source_revision: d578df6b6b71b8ce4b0e165f6e630705dc946bcefab55f8b43fc9cba6d60bb70
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new
---
# Decimal::new

初期化済み整数を作成します。個数や整数の金額など、符号付き 64 ビット整数に使うコンストラクターです。

```rust
fn new (value : i64) -> Self
```

## 引数と戻り値

`value: i64` → JavaScript の `bigint`。結果のスケールは 0 です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new" locale="ja" />

## 使い方の補足

JavaScript number ではなく `12n` を使ってください。i64 範囲外の値は bridge が拒否します。小数値には `new_from_string` を使います。

## 関連メソッド

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
