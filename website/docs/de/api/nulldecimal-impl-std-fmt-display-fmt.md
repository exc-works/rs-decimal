---
locale: de
source_revision: da8cebe24a220cbee0329a50db3c796d6390c270eb7d574cffaac17c4ddc1264
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::impl std :: fmt :: Display::fmt"
---
# NullDecimal::impl std :: fmt :: Display::fmt

Einen nullable-Wert mit Rust Display formatieren.

```rust
fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result
```

## Parameter, Rückgabe und übliche Verwendung

Display delegiert an nullable-string(): kanonischer innerer Text bei gültigem Wert, null bei ungültigem. Der Formatter schreibt Text, kein JSON-Token und keinen Bytepuffer. format! ruft Display::fmt auf; diese Trait-Syntax wird nativ ausgeführt und ist keine Browsermethode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-impl-std-fmt-display-fmt" locale="de" />

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
