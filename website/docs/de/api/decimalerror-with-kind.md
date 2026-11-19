---
locale: de
source_revision: ecc694dc9095e4bf91fee90f15b14f69b82899c7811b1a6abfdda76dd5e06c56
translation_provenance: en-US-german-reviewed
api_id: "DecimalError::with_kind"
---
# DecimalError::with_kind

Einen Fehler konsumieren und eine Kategorie hinzufügen.

```rust
fn with_kind (mut self , kind : ErrorKind) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

kind: ErrorKind; konsumiert self und liefert einen neuen DecimalError. Vorhandene Kategorien und Meldung bleiben erhalten, Duplikate werden ignoriert. Das ursprüngliche Browserhandle wird unbrauchbar. Ungültige JS-kind-Werte werden vor dem Move abgewiesen. Der benannte Browserkonstruktor erzeugt einen freigebbaren Rust-basierten Wert mit message, matches(), kinds() und with_kind(). Geworfene Bibliotheksausnahmen sind native JS-Error-Objekte mit category="library" und kinds-Array; verwenden Sie dort e.kinds.includes(ErrorKind.Unmarshal). Bridge-Fehler für parameter/resource/moved sind separat und besitzen keine Bibliotheksarten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-with-kind" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
