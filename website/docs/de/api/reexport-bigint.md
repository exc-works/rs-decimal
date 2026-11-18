---
locale: de
source_revision: 8110d2fd30808fd737c3f5730a5be2d19cf89f407f5d688859de476689342458
translation_provenance: en-US-german-reviewed
api_id: reexport::BigInt
---
# reexport::BigInt

BigInt aus der Crate-Wurzel importieren.

```rust
pub use num_bigint::BigInt as BigInt
```

## Parameter, Rückgabe und übliche Verwendung

Dies ist der öffentliche Rust-Reexport, kein neuer Typ und keine neue Implementierung. Die Importsyntax wird nativ getestet. Browserentsprechungen sind bigint, ein exaktes rationales Feldobjekt bzw. die globale Funktion most_significant_bit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-bigint" locale="de" />

## Grenzen im Browser

Ganzzahlen werden nicht über verlustbehaftete JS-Zahlen übertragen. Konvertierungstupel erhalten Rusts Reihenfolge Wert/Kennzeichen; false verspricht kein exaktes Ergebnis. Ausgeliehene Eingaben bleiben unverändert, Konstruktoren initialisieren Ergebnisse. Die [Ressourcenrichtlinie](../resource-policy) begrenzt Ganzzahleingaben, Ausgabeskala und rationales Koeffizientenwachstum getrennt von Bibliotheksfehlern. Rust-Importsyntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
