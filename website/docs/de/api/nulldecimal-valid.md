---
locale: de
source_revision: 2a36a5c46f57052b68644728359c44f15465f6bfca2c60d1540e40c603cc8c5b
translation_provenance: en-US-german-reviewed
api_id: NullDecimal.valid
---
# NullDecimal.valid

Anwesenheit lesen oder ändern, ohne die gespeicherte Dezimalzahl zu ändern.

```rust
valid: bool
```

## Übliche Verwendung und Semantik

Dieses öffentliche Rust-bool-Feld entspricht einem boolean-Getter/Setter. false löscht die Dezimalzahl nicht; true initialisiert sie nicht. Nichtboolesche Werte weist die JavaScript-API vor einer Änderung ab.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-valid" locale="de" />

Das gemeinsame Beispiel läuft in nativem Rust und im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
