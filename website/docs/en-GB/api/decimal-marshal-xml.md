---
locale: en-GB
source_revision: 5f41d4af42d404ec01165b3190db1cd2f96b9a84fa765bafbce34ae6b76aa2ef
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::marshal_xml"
---
# Decimal::marshal_xml

Get xml semantic content.

```rust
fn marshal_xml (& self) -> String
```

## Parameters, return and normal usage

No parameters; returns String/string. Rust requires the xml feature, enabled in the browser. Returns element character content; the caller supplies the element name, namespace and XML escaping. Canonical decimal text removes fractional trailing zeros. Uninitialized zero returns empty content; initialized zero returns "0". Reading never changes initialization or validity.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-xml" locale="en-GB" />

## Browser boundaries

Source input is capped at 8192 UTF-8 bytes. Before core numeric parsing, Rust checks the decoded first YAML scalar or XML element text, including escapes, CDATA, underscore normalization and YAML float zero insertion. YAML scalars permit at most 4096 hexadecimal digit characters to bound radix integer allocation; conservative checks may reject inputs accepted natively. Scale, coefficient and operation counts follow the [resource policy](../resource-policy). Moved handles cannot be read or mutated.

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
