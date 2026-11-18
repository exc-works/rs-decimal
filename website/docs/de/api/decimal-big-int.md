---
locale: de
source_revision: 1933adb169124e0ed9d429b21c95687e4fd59240647c4b0f684ca7bc76aabd2b
translation_provenance: en-US-german-reviewed
api_id: Decimal::big_int
---
# Decimal::big_int

Den exakten unskalierten Ganzzahlkoeffizienten kopieren.

```rust
fn big_int (& self) -> BigInt
```

## Parameter, Rückgabe und Hinweise

Rust liefert einen unabhängigen BigInt, die Bridge bigint. Anders als eine abschneidende Ganzzahlkonvertierung enthält dieser nachgestellte Nullen. Das Beispiel stellt bigint für JSON als Text dar.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-big-int" locale="de" />

## Fehler und Grenzen

Diese Methoden leihen Eingaben aus, ohne sie zu konsumieren oder zu verändern. Die Bibliotheksoperation liefert kein Result. Die Bridge weist freigegebene, verschobene oder fremden Läufen zugehörige Handles ab und wendet die [Ressourcenrichtlinie](../resource-policy) an. Beispiele verpacken skalare Ausgaben in einem `value`-Feld; Decimal-Ausgaben verwenden einen vollständigen Snapshot.

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
