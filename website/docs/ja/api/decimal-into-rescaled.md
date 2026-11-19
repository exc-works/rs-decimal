---
locale: ja
source_revision: aed80a094ad154f3abfbbdc72125d175f579082056ef3b224d8dda331e64cff5
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

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
