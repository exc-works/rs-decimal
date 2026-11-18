---
locale: ja
source_revision: f56b3cc9eb8667e7cf067d7d1256d88fafb84d3cb5b89748b59c7df428bab4cf
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::into_rescaled
---
# Decimal::into_rescaled

小数を消費し、小数点以下の桁数を変更します。

```rust
fn into_rescaled (self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

prec: isize、mode: RoundingMode。rescale と同様に丸めるかゼロを追加し、結果は常に初期化済みになります。Rust が InvalidPrecision、RoundUnnecessary、ブラウザーのリソースエラーを返してもレシーバーはムーブされます。JS の型や範囲が不正なら Rust に入る前に拒否するため、消費しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-rescaled" locale="ja" />

## ブラウザーの制約

上記の所有権規則に従ってください。ムーブ後のアクセスは moved エラーになり、free は冪等です。型とエイリアスの検証はムーブ前に行いますが、Rust のエラーは消費済みのレシーバーを復元しません。[リソースポリシー](../resource-policy)はライブラリエラーとは別です。累乗では中間値を、桁揃えでは係数の増大を確認します。保守的な検査により最終結果が小さくても拒否される場合があります。

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
