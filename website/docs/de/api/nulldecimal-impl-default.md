---
locale: de
source_revision: 71bc56362fd9f1c25a96932ba1ffd795db889517f58a951726bcd78e58fe2bc7
translation_provenance: en-US-german-reviewed
api_id: NullDecimal::impl Default
---
# NullDecimal::impl Default

Mit Rust Default einen fehlenden nullable-Wert erzeugen.

```rust
impl Default for NullDecimal
```

## Übliche Verwendung und Semantik

Gültigkeit und innere Initialisierung sind beide false. Die Bridge stellt NullDecimal.default() bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-default" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
