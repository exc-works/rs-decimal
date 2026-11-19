---
locale: de
source_revision: c654c9726a283aeb3b59ec2a73dcd6f45d404c2677f244d97a60cf2ff73a642b
translation_provenance: en-US-german-reviewed
api_id: NullDecimal::impl Default
---
# NullDecimal::impl Default

Mit Rust Default einen fehlenden nullable-Wert erzeugen.

```rust
impl Default for NullDecimal
```

## Übliche Verwendung und Semantik

Gültigkeit und innere Initialisierung sind beide false. Die JavaScript-API stellt NullDecimal.default() bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-default" locale="de" />

Dieses Rust-Syntaxbeispiel läuft in nativen Tests und behauptet keine Trait-Unterstützung im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
