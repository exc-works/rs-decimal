---
locale: ja
source_revision: 3c6ebaee1d6c2d15a7801caee0a8597128eea9b110eb2748ac00d60cf69e23fd
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::scan"
---
# NullDecimal::scan

ドライバーに依存しない SQL スカラーを読み取ります。

```rust
fn scan (& mut self , value : ScanValue < '_ >) -> Result < () , DecimalError >
```

## 引数、戻り値、通常の使い方

value: ScanValue。レシーバーを変更し、`Result<(), DecimalError>`/undefined を返します。バリアントを明示的に選択してください。Int64 は正確な bigint、Float32 は binary32 への変換、Text/Bytes は元の小数桁の保持を使います。SQL テキストの外側には対応する二重引用符 1 組を置け、JSON エスケープ解除なしで除去します。NULL は有効性を解除して内部 Decimal を未初期化のゼロにリセットし、null 以外の入力が成功すると valid=true にします。未対応の型と非有限の浮動小数点は Unmarshal です。他のデコード失敗は基底の種類を保持します。すべてのエラーでレシーバーの状態を保持します。このアダプターにデータベース接続は不要です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-scan" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
