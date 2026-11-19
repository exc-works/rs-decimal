---
locale: ja
source_revision: c526a43d69654fb21b49f56d858016f45f88e7f7f6ec8eaa0c13c3e5e939cd03
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::uint64
---
# Decimal::uint64

成功フラグとともに固定幅整数へ変換します。

```rust
fn uint64 (& self) -> (u64 , bool)
```

## 引数、戻り値、通常の使い方

引数なし。(u64, bool) を返し、[bigint, boolean] に対応します。範囲内の整数値は整数と true を返します。小数やオーバーフローは切り捨て値や例外ではなく (0, false) を返します。負の値も (0, false) を返します。スケールだけでは整数か判定できず、12.00 は成功します。失敗と有効なゼロを区別するため、必ずフラグを確認してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-uint64" locale="ja" />

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
