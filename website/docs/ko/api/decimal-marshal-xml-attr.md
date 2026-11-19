---
locale: ko
source_revision: 8ca020b2d56652a32165b4756446630d2a88be60a068a89e4f65ce218930abe5
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::marshal_xml_attr"
---
# Decimal::marshal_xml_attr

xml_attr의 의미상 내용을 얻습니다.

```rust
fn marshal_xml_attr (& self) -> String
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. String/string을 반환합니다. Rust는 xml 기능이 필요하며 브라우저에서 활성화되어 있습니다. 이미 디코딩된 속성 내용을 반환하며 호출자가 속성 이름과 XML 이스케이프를 처리합니다. 정규 소수 텍스트는 소수 부분의 후행 0을 제거합니다. 미초기화 0은 빈 내용을, 초기화된 0은 "0"을 반환합니다. 읽기는 초기화나 유효성을 바꾸지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-xml-attr" locale="ko" />

## 관련 메서드

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
