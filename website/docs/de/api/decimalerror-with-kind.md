---
locale: de
source_revision: f5327cbdbb40212ac4046096b90742ab088011d23930faef1e12c326d8e03e09
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

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
