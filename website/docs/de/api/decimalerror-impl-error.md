---
locale: de
source_revision: d94c5a1a48ed0fbe522456d88bc4774ce805cace3b32e2a5428e2527dc3be2ce
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

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
