---
locale: de
source_revision: 473b8cc7f11457cef7cc926925c4f230754afa71c4dea38c85c3b954c8ca89fd
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::impl Copy"
---
# ErrorKind::impl Copy

Die Rust-Implementierung von Copy verwenden.

```rust
impl Copy for ErrorKind
```

## Parameter, Rückgabe und übliche Verwendung

Copy dupliziert dieses kleine Enum; ausgeliehene Skalardaten werden nicht tief kopiert. Dies ist native Rust-Trait-Syntax; der Browser stellt sie nicht als Methode bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-copy" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
