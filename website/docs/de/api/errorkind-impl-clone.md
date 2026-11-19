---
locale: de
source_revision: b4b7e965750a7655fdeb57385feedb3c62874c4ab66fdfb6a3f2062d579960d6
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::impl Clone"
---
# ErrorKind::impl Clone

Die Rust-Implementierung von Clone verwenden.

```rust
impl Clone for ErrorKind
```

## Parameter, Rückgabe und übliche Verwendung

Clone erhält den vollständigen Wert; ausgeliehene ScanValue-Slices verweisen weiterhin auf die ursprünglichen Daten. Dies ist native Rust-Trait-Syntax; der Browser stellt sie nicht als Methode bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-clone" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
