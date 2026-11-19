---
locale: ja
source_revision: dd3a6777ec0d2daf335730b8ca67dd44bd500850bf5f5a894aa8a18b1d5c40f9
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new_from_int
---
# Decimal::new_from_int

ネイティブ幅の符号付き整数から初期化済み整数を作成します。

```rust
fn new_from_int (value : isize) -> Self
```

## 引数と戻り値

`value: isize` → 整数の JavaScript `number`。結果のスケールは 0 です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-int" locale="ja" />

## 使い方の補足

ブラウザーのターゲットは wasm32 なので、ネイティブ Rust が 64 ビット環境でも、受け付ける範囲は -2147483648 から 2147483647 です。小数、NaN、無限大は引数エラーになります。

## 関連メソッド

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
