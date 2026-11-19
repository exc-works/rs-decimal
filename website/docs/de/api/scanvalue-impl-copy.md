---
locale: de
source_revision: 7eeabc8ee1ace4f862b185d7c83f7d0a53594b9ff67ac15d988dde4ffaa12e4b
translation_provenance: en-US-german-reviewed
api_id: "ScanValue::impl Copy"
---
# ScanValue::impl Copy

Die Rust-Implementierung von Copy verwenden.

```rust
impl Copy for ScanValue
```

## Parameter, Rückgabe und übliche Verwendung

Copy dupliziert dieses kleine Enum; ausgeliehene Skalardaten werden nicht tief kopiert. Dies ist native Rust-Trait-Syntax; der Browser stellt sie nicht als Methode bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-copy" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
