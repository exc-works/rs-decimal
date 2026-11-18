---
locale: de
source_revision: 751751d0b535375fecff33d2510df43e92ffca892248b1be4dba2a34ce1d0f99
translation_provenance: en-US-german-reviewed
api_id: zero
---
# zero

Eine initialisierte Null mit Skala null konstruieren.

```rust
fn zero () -> Decimal
```

## Parameter, Rückgabe und Hinweise

Keine Parameter; liefert einen neuen Decimal. Anders als Decimal::default() ist das Ergebnis initialisiert, obwohl beide numerisch gleich sind.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-zero" locale="de" />

## Grenzen

Die Bridge erhält Rust-Namen und Parameterreihenfolge. Freie Funktionen stehen im Playground direkt bereit. Ausgeliehene Eingaben werden weder konsumiert noch verändert; ungültige Handles oder solche aus fremden Läufen werden abgewiesen. Alle Operationen folgen der [Ressourcenrichtlinie](../resource-policy).

## Verwandte Methoden

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
