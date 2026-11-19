---
locale: en-US
source_revision: 87ff1c64a9a30b8c362e1bf8438b7f751aeb4f561a2bd311095a9f2b86ed45db
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

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
