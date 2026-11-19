---
locale: en-US
source_revision: f9048a3e7634cd109eea9581928de5e077f7c7324b6f3b621f732137009253fb
api_id: "NullDecimal::unmarshal_xml_attr"
---
# NullDecimal::unmarshal_xml_attr

Decode xml_attr into an existing value.

```rust
fn unmarshal_xml_attr (& mut self , value : & str) -> Result < () , DecimalError >
```

## Parameters, return and normal usage

value: &str/string; mutates the receiver and returns `Result<(), DecimalError>`/undefined. Rust requires xml. Pass the already entity-decoded attribute value, not an XML attribute fragment: "1.23" is valid but `"&#49;.23"` is not decoded again. Trims surrounding whitespace and parses exact decimal text. Empty content clears valid and resets the inner Decimal to uninitialized zero. Successful non-null decoding initializes the decimal and preserves nonzero trailing scale. It sets valid=true. Errors preserve all receiver fields and retain library error kinds including Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-xml-attr" locale="en-US" />

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
