---
locale: ja
source_revision: 91bb241ee917c58402441fd00f901b8aa75a693fa5e9137596c74ca66e946c7e
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::int64
---
# Decimal::int64

成功フラグとともに固定幅整数へ変換します。

```rust
fn int64 (& self) -> (i64 , bool)
```

## 引数、戻り値、通常の使い方

引数なし。(i64, bool) を返し、[bigint, boolean] に対応します。範囲内の整数値は整数と true を返します。小数やオーバーフローは切り捨て値や例外ではなく (0, false) を返します。スケールだけでは整数か判定できず、12.00 は成功します。失敗と有効なゼロを区別するため、必ずフラグを確認してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-int64" locale="ja" />

## ブラウザーの制約

整数の転送は精度を失う JS number を経由しません。変換タプルは Rust の値／フラグの順序を保持します。false は正確な結果を保証しません。借用入力は変わらず、コンストラクターは結果を初期化します。[リソースポリシー](../resource-policy)はライブラリエラーとは別に、整数入力、出力スケール、有理数の係数増大を制限します。Rust のインポート構文はネイティブで検証するものとして明示します。

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
