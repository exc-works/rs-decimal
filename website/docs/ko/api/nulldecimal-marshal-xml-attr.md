---
locale: ko
source_revision: b8d3cac3d5f9251c9158f0efb63e99a5a338182fad355d14b4c984c245247f04
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::marshal_xml_attr"
---
# NullDecimal::marshal_xml_attr

xml_attr의 의미상 내용을 얻습니다.

```rust
fn marshal_xml_attr (& self) -> Option < String >
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. `Option<String>`/string | null을 반환합니다. Rust는 xml 기능이 필요하며 브라우저에서 활성화되어 있습니다. 이미 디코딩된 속성 내용을 반환하며 호출자가 속성 이름과 XML 이스케이프를 처리합니다. 정규 소수 텍스트는 소수 부분의 후행 0을 제거합니다. 무효인 nullable은 None/null을 반환하므로 빈 속성을 쓰지 말고 속성을 생략하세요. 유효하며 내부가 미초기화 0인 값은 "0"을 반환합니다. 읽기는 초기화나 유효성을 바꾸지 않으며 브라우저는 Rust None을 null에 대응시킵니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-xml-attr" locale="ko" />

## 관련 메서드

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
