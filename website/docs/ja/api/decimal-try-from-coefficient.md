---
locale: ja
source_revision: 34aa17297c2ce6b5b73e997c910f57874da73c70f9cf52cd3fadd2ab710a529d
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::try_from_coefficient
---
# Decimal::try_from_coefficient

所有する係数とスケールから初期化済みの値を構築します。

```rust
fn try_from_coefficient (coefficient : BigInt , scale : usize) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

coefficient: BigInt、scale: usize（非負の小数点以下の桁数）。末尾のゼロと、値がゼロのときのスケールを保持し、テキスト解析の正規化は使いません。isize::MAX を超えると Rust は Overflow を返します。ブラウザーは正確な bigint と wasm32 usize 範囲内の整数 number を使います。スケールが i32::MAX を超えると Overflow を保ち、それ以外で 1000 を超えるスケールは playground のリソース上限を超えます。JS bigint は不変なので、ムーブされるラッパーではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-try-from-coefficient" locale="ja" />

## ブラウザーの制約

上記の所有権規則に従ってください。ムーブ後のアクセスは moved エラーになり、free は冪等です。型とエイリアスの検証はムーブ前に行いますが、Rust のエラーは消費済みのレシーバーを復元しません。[リソースポリシー](../resource-policy)はライブラリエラーとは別です。累乗では中間値を、桁揃えでは係数の増大を確認します。保守的な検査により最終結果が小さくても拒否される場合があります。

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
