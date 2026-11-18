---
locale: en-US
source_revision: e89c105d8071261190fddaa241c9f1fa55981badd4bbdc0f74617c9bd940eb39
api_id: "NullDecimal::marshal_xml"
---
# NullDecimal::marshal_xml

Get xml semantic content.

```rust
fn marshal_xml (& self) -> String
```

## Parameters, return and normal usage

No parameters; returns String/string. Rust requires the xml feature, enabled in the browser. Returns element character content; the caller supplies the element name, namespace and XML escaping. Canonical decimal text removes fractional trailing zeros. Invalid nullable values return empty content; valid inner uninitialized zero returns "0". Reading never changes initialization or validity.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-xml" locale="en-US" />

## Browser boundaries

Source input is capped at 8192 UTF-8 bytes. Before core numeric parsing, Rust checks the decoded first YAML scalar or XML element text, including escapes, CDATA, underscore normalization and YAML float zero insertion. YAML scalars permit at most 4096 hexadecimal digit characters to bound radix integer allocation; conservative checks may reject inputs accepted natively. Scale, coefficient and operation counts follow the [resource policy](../resource-policy). Moved handles cannot be read or mutated.

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
