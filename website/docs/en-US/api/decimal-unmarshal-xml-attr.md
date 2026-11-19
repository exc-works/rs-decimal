---
locale: en-US
source_revision: 8e284228528617403ea37afb52c965d21d076aee976dac5cd366e305507e8e05
api_id: "Decimal::unmarshal_xml_attr"
---
# Decimal::unmarshal_xml_attr

Decode xml_attr into an existing value.

```rust
fn unmarshal_xml_attr (& mut self , value : & str) -> Result < () , DecimalError >
```

## Parameters, return and normal usage

value: &str/string; mutates the receiver and returns `Result<(), DecimalError>`/undefined. Rust requires xml. Pass the already entity-decoded attribute value, not an XML attribute fragment: "1.23" is valid but `"&#49;.23"` is not decoded again. Trims surrounding whitespace and parses exact decimal text. Empty content resets the receiver to uninitialized zero. Successful non-null decoding initializes the decimal and preserves nonzero trailing scale. Errors preserve all receiver fields and retain library error kinds including Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-unmarshal-xml-attr" locale="en-US" />

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
