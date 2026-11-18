---
locale: ja
source_revision: cd0b0af7153d96e3d2f3f7126c5b673c301639179f9c4225b3dd81e2a74678a4
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::add_owned
---
# Decimal::add_owned

左オペランドを消費して正確な演算を行います。

```rust
fn add_owned (self , other : & Self) -> Self
```

## 引数、戻り値、通常の使い方

other: &Decimal。入力の大きい方のスケールで初期化済み Decimal を返します。レシーバーはムーブされ、other は引き続き使用できます。記憶領域は再利用できますが、増大時には割り当てが発生することがあります。bridge は同じラッパーをレシーバーと other の両方に渡すことをムーブ前に拒否します。Rust に入ると、リソースエラーでもレシーバーを消費します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-add-owned" locale="ja" />

## ブラウザーの制約

上記の所有権規則に従ってください。ムーブ後のアクセスは moved エラーになり、free は冪等です。型とエイリアスの検証はムーブ前に行いますが、Rust のエラーは消費済みのレシーバーを復元しません。[リソースポリシー](../resource-policy)はライブラリエラーとは別です。累乗では中間値を、桁揃えでは係数の増大を確認します。保守的な検査により最終結果が小さくても拒否される場合があります。

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
