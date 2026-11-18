---
locale: de
source_revision: a66891e5a69e79ad1f199a0dd34da946899f3deff1662608f76ae3c569145632
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

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
