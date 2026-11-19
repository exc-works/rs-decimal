---
locale: de
source_revision: 65435d6f54d8777702cf2f629258b85e312b71241d8a0725f7c209d6e7b4a200
translation_provenance: en-US-german-reviewed
api_id: "PRECISION_FIXED_SIZE"
---
# PRECISION_FIXED_SIZE

Die Bytelänge des binären Präzisionspräfixes lesen.

```rust
const PRECISION_FIXED_SIZE: usize = 4
```

## Parameter, Rückgabe und übliche Verwendung

Eine Rust-usize-Konstante mit Wert 4: die Bytelänge des Big-Endian-Skalenpräfixes im Go-kompatiblen Binärformat. Sie ist weder Dezimalziffernzahl noch Präzisionsgrenze. Der Browser stellt eine schreibgeschützte number aus Rust bereit. Der Reexport an der Crate-Wurzel ist Rust-Importsyntax.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-precision-fixed-size" locale="de" />

## Verwandte Methoden

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
