---
locale: de
source_revision: d57c1e8ddb07024ed3c5c002f095b2cee086c1a0719df70a0b4ba8761f29418d
translation_provenance: en-US-german-reviewed
api_id: NullDecimal.valid
---
# NullDecimal.valid

Anwesenheit lesen oder ändern, ohne die gespeicherte Dezimalzahl zu ändern.

```rust
valid: bool
```

## Übliche Verwendung und Semantik

Dieses öffentliche Rust-bool-Feld entspricht einem boolean-Getter/Setter. false löscht die Dezimalzahl nicht; true initialisiert sie nicht. Nichtboolesche Werte weist die Bridge vor einer Änderung ab.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-valid" locale="de" />

Das gemeinsame Beispiel läuft in nativem Rust und im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
