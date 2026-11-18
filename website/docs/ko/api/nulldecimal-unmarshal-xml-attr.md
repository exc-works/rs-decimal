---
locale: ko
source_revision: 72730d41ab3e71fa242fbf48a88a0c6a502506e0eb80815d9391890ebfff87d8
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::unmarshal_xml_attr"
---
# NullDecimal::unmarshal_xml_attr

xml_attr을 기존 값으로 디코딩합니다.

```rust
fn unmarshal_xml_attr (& mut self , value : & str) -> Result < () , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

value: &str/string. 수신 객체를 바꾸고 `Result<(), DecimalError>`/undefined를 반환합니다. Rust는 xml이 필요합니다. XML 속성 조각이 아니라 엔터티 디코딩이 끝난 속성값을 전달하세요. "1.23"은 유효하지만 `"&#49;.23"`은 다시 디코딩하지 않습니다. 앞뒤 공백을 제거하고 정확한 소수 텍스트를 파싱합니다. 내용이 비면 valid를 해제하고 내부 Decimal을 미초기화 0으로 재설정합니다. null이 아닌 입력의 디코딩이 성공하면 소수를 초기화하고 0이 아닌 수치의 후행 자릿수를 유지합니다. valid=true로 설정합니다. 오류는 수신 객체의 모든 필드를 유지하고 Unmarshal을 포함한 라이브러리 오류 종류를 유지합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-xml-attr" locale="ko" />

## 브라우저 제한

소스는 UTF-8 8192바이트까지입니다. 코어 수치 파싱 전에 Rust는 디코딩된 첫 YAML 스칼라 또는 XML 요소 텍스트를 검사하며 이스케이프, CDATA, 밑줄 정규화, YAML 실수의 0 삽입도 포함합니다. YAML 스칼라는 진법별 정수 할당을 제한하기 위해 최대 4096개 16진 숫자 문자를 허용합니다. 보수적인 검사로 네이티브가 수락하는 입력도 거부할 수 있습니다. 소수 자릿수, 계수, 연산 횟수는 [리소스 정책](../resource-policy)을 따릅니다. 이동된 핸들은 읽거나 바꿀 수 없습니다.

## 관련 메서드

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
