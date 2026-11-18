---
locale: en-US
source_revision: 2596ef8441aa7ec726abb87c7af62732ec4f2729da79754eba71b3cb01735596
api_id: "Decimal::marshal_yaml"
---
# Decimal::marshal_yaml

Get yaml semantic content.

```rust
fn marshal_yaml (& self) -> String
```

## Parameters, return and normal usage

No parameters; returns String/string. Rust requires the yaml feature, enabled in the browser. Returns a semantic YAML string scalar, not a serialized YAML document. Pass it to a YAML emitter as a string so quoting/tagging is handled correctly. Canonical decimal text removes fractional trailing zeros. Default uninitialized zero returns "0". Reading never changes initialization or validity.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-yaml" locale="en-US" />

## Browser boundaries

Source input is capped at 8192 UTF-8 bytes. Before core numeric parsing, Rust checks the decoded first YAML scalar or XML element text, including escapes, CDATA, underscore normalization and YAML float zero insertion. YAML scalars permit at most 4096 hexadecimal digit characters to bound radix integer allocation; conservative checks may reject inputs accepted natively. Scale, coefficient and operation counts follow the [resource policy](../resource-policy). Moved handles cannot be read or mutated.

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
