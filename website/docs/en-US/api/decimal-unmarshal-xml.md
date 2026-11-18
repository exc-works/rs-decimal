---
locale: en-US
source_revision: 1640bba425b4f674d29468bfc500f4a4319c227e5d4c34f98666dafad0d9b981
api_id: "Decimal::unmarshal_xml"
---
# Decimal::unmarshal_xml

Decode xml into an existing value.

```rust
fn unmarshal_xml (& mut self , source : & str) -> Result < () , DecimalError >
```

## Parameters, return and normal usage

source: &str/string; mutates the receiver and returns `Result<(), DecimalError>`/undefined. Rust requires xml. Decodes the first XML element, combining its direct text, CDATA and decoded entity references. Nested child text is ignored; text before/after a child is concatenated. Later elements are ignored. Names, attributes, character references and matched closing tags are validated by the core. Trimmed empty content resets the receiver to uninitialized zero; this differs from YAML null, which leaves it unchanged. Successful non-null decoding initializes the decimal and preserves nonzero trailing scale. Errors preserve all receiver fields and retain library error kinds including Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-unmarshal-xml" locale="en-US" />

## Browser boundaries

Source input is capped at 8192 UTF-8 bytes. Before core numeric parsing, Rust checks the decoded first YAML scalar or XML element text, including escapes, CDATA, underscore normalization and YAML float zero insertion. YAML scalars permit at most 4096 hexadecimal digit characters to bound radix integer allocation; conservative checks may reject inputs accepted natively. Scale, coefficient and operation counts follow the [resource policy](../resource-policy). Moved handles cannot be read or mutated.

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
