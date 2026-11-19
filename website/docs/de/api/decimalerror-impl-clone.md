---
locale: de
source_revision: 9dbbcca20e9ec70948f275a66820b5aebd2bfa7f33ae4f382b6d22c025c682c3
translation_provenance: en-US-german-reviewed
api_id: "DecimalError::impl Clone"
---
# DecimalError::impl Clone

Die Rust-Implementierung von Clone verwenden.

```rust
impl Clone for DecimalError
```

## Parameter, Rückgabe und übliche Verwendung

Clone erhält den vollständigen Wert; ausgeliehene ScanValue-Slices verweisen weiterhin auf die ursprünglichen Daten. Dies ist native Rust-Trait-Syntax; der Browser stellt sie nicht als Methode bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-clone" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
