---
locale: de
source_revision: 51d80d897f775a7466e06561586b9d500a34859b5d00dfee1f00a368a7a33f1e
translation_provenance: en-US-german-reviewed
api_id: "DecimalError::kinds"
---
# DecimalError::kinds

Alle Fehlerkategorien lesen.

```rust
fn kinds (& self) -> & [ErrorKind]
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert &[ErrorKind]/ein unabhängiges Array von Variantennamen. Kategorien werden dedupliziert und nach Rust-Enumreihenfolge statt alphabetisch sortiert. Der benannte Browserkonstruktor erzeugt einen freigebbaren Rust-basierten Wert mit message, matches(), kinds() und with_kind(). Geworfene Bibliotheksausnahmen sind native JS-Error-Objekte mit category="library" und kinds-Array; verwenden Sie dort e.kinds.includes(ErrorKind.Unmarshal). Bridge-Fehler für parameter/resource/moved sind separat und besitzen keine Bibliotheksarten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-kinds" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
