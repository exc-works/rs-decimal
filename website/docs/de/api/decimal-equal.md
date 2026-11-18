---
locale: de
source_revision: 6d86e4a4d53be90de2e3c79ca0231980d419a71194ef67f300c06ca432f017c2
translation_provenance: en-US-german-reviewed
api_id: Decimal::equal
---
# Decimal::equal

Prüfen, ob zwei Dezimalzahlen denselben Zahlenwert darstellen.

```rust
fn equal (& self , other : & Self) -> bool
```

## Parameter, Rückgabe und Hinweise

other: &Decimal ausleihen; liefert bool/boolean. Skala und Initialisierung werden ignoriert. JavaScript === vergleicht die Identität der Wrapper; equal() prüft die numerische Gleichheit von Rust.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-equal" locale="de" />

## Fehler und Grenzen

Diese Methoden leihen Eingaben aus, ohne sie zu konsumieren oder zu verändern. Die Bibliotheksoperation liefert kein Result. Die Bridge weist freigegebene, verschobene oder fremden Läufen zugehörige Handles ab und wendet die [Ressourcenrichtlinie](../resource-policy) an. Beispiele verpacken skalare Ausgaben in einem `value`-Feld; Decimal-Ausgaben verwenden einen vollständigen Snapshot.

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
