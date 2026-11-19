---
locale: de
source_revision: d3605c64b7e680ec58960c25358a321ed68015b81953ff878939d179147793d3
translation_provenance: en-US-german-reviewed
api_id: "DecimalError::impl Error"
---
# DecimalError::impl Error

Die Rust-Implementierung von Error verwenden.

```rust
impl Error for DecimalError
```

## Parameter, Rückgabe und übliche Verwendung

Als Standard-Rust-Error verwenden, einschließlich Weitergabe in einer Box. Zusätzliche Kategorien sind keine verkettete Ursache; source() ist None. Dies ist native Rust-Trait-Syntax; der Browser stellt sie nicht als Methode bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-error" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
