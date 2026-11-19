---
locale: de
source_revision: 1b410a44c40ddf1ed7ab08ecafd94bf955a8c6e8ace55716beba7530f0db9122
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::impl Debug"
---
# ErrorKind::impl Debug

Die Rust-Implementierung von Debug verwenden.

```rust
impl Debug for ErrorKind
```

## Parameter, Rückgabe und übliche Verwendung

Debug liefert Diagnosetext, keine stabile Übertragungsserialisierung. Dies ist native Rust-Trait-Syntax; der Browser stellt sie nicht als Methode bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-debug" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
