---
locale: en-GB
source_revision: 634992be26686e49442a948ce62a476a4bcc34db498b408945e4ad08ce9f7530
translation_provenance: en-US-reviewed-reuse
api_id: "NullDecimal::marshal_yaml"
---
# NullDecimal::marshal_yaml

Get yaml semantic content.

```rust
fn marshal_yaml (& self) -> Option < String >
```

## Parameters, return and normal usage

No parameters; returns `Option<String>`/string | null. Rust requires the yaml feature, enabled in the browser. Returns a semantic YAML string scalar, not a serialized YAML document. Pass it to a YAML emitter as a string so quoting/tagging is handled correctly. Canonical decimal text removes fractional trailing zeros. Invalid nullable values return None/null, representing a YAML null scalar; valid values with an uninitialized inner zero return "0". Reading never changes initialization or validity; the browser maps Rust None to null.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-yaml" locale="en-GB" />

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
