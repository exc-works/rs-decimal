---
locale: ja
source_revision: 4d2afeddcd523e94b9e9276fe6f8c19193bfe95254c1ef9ded67b301a566152b
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::min_ref
---
# Decimal::min_ref

数値の比較によって元のオペランドを選択します。

```rust
fn min_ref < 'a > (& 'a self , other : & 'a Self) -> & 'a Self
```

## 引数、戻り値、通常の使い方

other: &Decimal。Rust は入力と同じライフタイムの共有参照を返します。値が等しい場合は self を選び、スケールと初期化状態を保持します。bridge はコピーではなく、対応する元のラッパーを返します。そのラッパーをムーブまたは解放すると、すべての JS エイリアスも無効になります。JS は Rust の借用ライフタイムを強制できません。独立して所有する結果には max/min を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-min-ref" locale="ja" />

## ブラウザーの制約

上記の所有権規則に従ってください。ムーブ後のアクセスは moved エラーになり、free は冪等です。型とエイリアスの検証はムーブ前に行いますが、Rust のエラーは消費済みのレシーバーを復元しません。[リソースポリシー](../resource-policy)はライブラリエラーとは別です。累乗では中間値を、桁揃えでは係数の増大を確認します。保守的な検査により最終結果が小さくても拒否される場合があります。

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
