---
locale: en-US
source_revision: 14c3a4a68a793992e79fa29a86d0ef2a5b8427f3e5ce4523962a0b1a25a66098
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

<MethodExample id="yaml-xml-nulldecimal-marshal-yaml" locale="en-US" />

## Browser boundaries

Source input is capped at 8192 UTF-8 bytes. Before core numeric parsing, Rust checks the decoded first YAML scalar or XML element text, including escapes, CDATA, underscore normalization and YAML float zero insertion. YAML scalars permit at most 4096 hexadecimal digit characters to bound radix integer allocation; conservative checks may reject inputs accepted natively. Scale, coefficient and operation counts follow the [resource policy](../resource-policy). Moved handles cannot be read or mutated.

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
