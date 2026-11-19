---
locale: ja
source_revision: c991ff009b87a451c4d3a01a7e163c4b8e995107fc18ca0a0ef4ec120e3aee71
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::min_ref
---
# Decimal::min_ref

数値の比較によって元のオペランドを選択します。

```rust
fn min_ref < 'a > (& 'a self , other : & 'a Self) -> & 'a Self
```

## 引数、戻り値、通常の使い方

other: &Decimal。Rust は入力と同じライフタイムの共有参照を返します。値が等しい場合は self を選び、スケールと初期化状態を保持します。JavaScript はコピーではなく、対応する元のラッパーを返します。そのラッパーをムーブまたは解放すると、すべての JS エイリアスも無効になります。JS は Rust の借用ライフタイムを強制できません。独立して所有する結果には max/min を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-min-ref" locale="ja" />

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
