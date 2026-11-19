---
locale: ko
source_revision: 634992be26686e49442a948ce62a476a4bcc34db498b408945e4ad08ce9f7530
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::marshal_yaml"
---
# NullDecimal::marshal_yaml

yaml의 의미상 내용을 얻습니다.

```rust
fn marshal_yaml (& self) -> Option < String >
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. `Option<String>`/string | null을 반환합니다. Rust는 yaml 기능이 필요하며 브라우저에서 활성화되어 있습니다. 직렬화된 YAML 문서가 아니라 의미상 YAML 문자열 스칼라를 반환합니다. 따옴표와 태그가 올바르게 처리되도록 YAML 출력기에 문자열로 전달하세요. 정규 소수 텍스트는 소수 부분의 후행 0을 제거합니다. 무효인 nullable은 YAML null 스칼라를 나타내는 None/null을 반환합니다. 유효하며 내부가 미초기화 0인 값은 "0"을 반환합니다. 읽기는 초기화나 유효성을 바꾸지 않으며 브라우저는 Rust None을 null에 대응시킵니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-yaml" locale="ko" />

## 관련 메서드

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
