---
locale: de
source_revision: fe0a2cbd7e86aa455f6274096dc78e80786ab081e8fd78826b22f34970a7dfa9
translation_provenance: en-US-german-reviewed
api_id: reexport::most_significant_bit
---
# reexport::most_significant_bit

most_significant_bit aus der Crate-Wurzel importieren.

```rust
pub use conversion::most_significant_bit as most_significant_bit
```

## Parameter, Rückgabe und übliche Verwendung

Dies ist der öffentliche Rust-Reexport, kein neuer Typ und keine neue Implementierung. Die Importsyntax wird nativ getestet. Browserentsprechungen sind bigint, ein exaktes rationales Feldobjekt bzw. die globale Funktion most_significant_bit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-most-significant-bit" locale="de" />

## Grenzen im Browser

Ganzzahlen werden nicht über verlustbehaftete JS-Zahlen übertragen. Konvertierungstupel erhalten Rusts Reihenfolge Wert/Kennzeichen; false verspricht kein exaktes Ergebnis. Ausgeliehene Eingaben bleiben unverändert, Konstruktoren initialisieren Ergebnisse. Die [Ressourcenrichtlinie](../resource-policy) begrenzt Ganzzahleingaben, Ausgabeskala und rationales Koeffizientenwachstum getrennt von Bibliotheksfehlern. Rust-Importsyntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
