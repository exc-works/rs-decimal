---
locale: de
source_revision: 4e9262b5d73b0bfa6dbb9a564894dae85973da389d298cf19e5836d09170ed0d
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::impl std :: fmt :: Display"
---
# NullDecimal::impl std :: fmt :: Display

Einen nullable-Wert mit Rust Display formatieren.

```rust
impl std :: fmt :: Display for NullDecimal
```

## Parameter, Rückgabe und übliche Verwendung

Display delegiert an nullable-string(): kanonischer innerer Text bei gültigem Wert, null bei ungültigem. Der Formatter schreibt Text, kein JSON-Token und keinen Bytepuffer. format! ruft Display::fmt auf; diese Trait-Syntax wird nativ ausgeführt und ist keine Browsermethode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-impl-std-fmt-display" locale="de" />

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
