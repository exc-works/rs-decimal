---
locale: de
source_revision: e7411932b8884630967e9c3e34bf9db436dc8920df8139ae778ac83fc80fdcb3
translation_provenance: en-US-german-reviewed
api_id: "DecimalError"
---
# DecimalError

Eine Meldung und eine oder mehrere Bibliotheksfehlerarten behalten.

```rust
pub struct DecimalError
```

## Parameter, Rückgabe und übliche Verwendung

DecimalError speichert eine lesbare Meldung und geordnete, deduplizierte Kategorien. with_kind konsumiert den Wert; matches und kinds leihen ihn aus. Er implementiert Rust Error und Display. Der benannte Browserkonstruktor erzeugt einen freigebbaren Rust-basierten Wert mit message, matches(), kinds() und with_kind(). Geworfene Bibliotheksausnahmen sind native JS-Error-Objekte mit category="library" und kinds-Array; verwenden Sie dort e.kinds.includes(ErrorKind.Unmarshal). Bridge-Fehler für parameter/resource/moved sind separat und besitzen keine Bibliotheksarten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
