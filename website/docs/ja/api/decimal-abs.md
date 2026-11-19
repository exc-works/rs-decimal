---
locale: ja
source_revision: c71c6b4ce225ea223944d584a5ec03b1adfe1dbd2adeb04f6beb38800f1e252e
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::abs
---
# Decimal::abs

絶対値を持つ独立した小数を作成します。

```rust
fn abs (& self) -> Self
```

## 引数、戻り値、使用上の注意

Decimal を返し、スケールと末尾のゼロを保持します。入力が既定値でも結果は初期化済みになり、入力は変更しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-abs" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
