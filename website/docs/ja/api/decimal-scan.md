---
locale: ja
source_revision: b5003d3283929da01970193aca2fe128bcc80043185972b8b1e270211629771c
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::scan"
---
# Decimal::scan

ドライバーに依存しない SQL スカラーを読み取ります。

```rust
fn scan (& mut self , value : ScanValue < '_ >) -> Result < () , DecimalError >
```

## 引数、戻り値、通常の使い方

value: ScanValue。レシーバーを変更し、`Result<(), DecimalError>`/undefined を返します。バリアントを明示的に選択してください。Int64 は正確な bigint、Float32 は binary32 への変換、Text/Bytes は元の小数桁の保持を使います。SQL テキストの外側には対応する二重引用符 1 組を置け、JSON エスケープ解除なしで除去します。NULL はレシーバーを未初期化のゼロにリセットします。未対応の型と非有限の浮動小数点は Unmarshal です。他のデコード失敗は基底の種類を保持します。すべてのエラーでレシーバーの状態を保持します。このアダプターにデータベース接続は不要です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-scan" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
