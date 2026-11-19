---
locale: en-US
source_revision: 25be20babe80f8d8b33e901842da879899c54ff8a78f97427ea80f9ca18a005d
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

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
