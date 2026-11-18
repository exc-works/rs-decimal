---
locale: ko
source_revision: 1640bba425b4f674d29468bfc500f4a4319c227e5d4c34f98666dafad0d9b981
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::unmarshal_xml"
---
# Decimal::unmarshal_xml

xml을 기존 값으로 디코딩합니다.

```rust
fn unmarshal_xml (& mut self , source : & str) -> Result < () , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

source: &str/string. 수신 객체를 바꾸고 `Result<(), DecimalError>`/undefined를 반환합니다. Rust는 xml이 필요합니다. 첫 XML 요소를 디코딩하여 직접 텍스트, CDATA, 디코딩된 엔터티 참조를 합칩니다. 중첩 자식의 텍스트는 무시하고 자식 전후의 텍스트를 연결합니다. 이후 요소는 무시합니다. 이름, 속성, 문자 참조, 닫는 태그 일치는 코어가 검사합니다. 앞뒤 공백을 제거한 내용이 비면 수신 객체를 미초기화 0으로 재설정합니다. 변경하지 않는 YAML null과 다릅니다. null이 아닌 입력의 디코딩이 성공하면 소수를 초기화하고 0이 아닌 수치의 후행 자릿수를 유지합니다. 오류는 수신 객체의 모든 필드를 유지하고 Unmarshal을 포함한 라이브러리 오류 종류를 유지합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-unmarshal-xml" locale="ko" />

## 브라우저 제한

소스는 UTF-8 8192바이트까지입니다. 코어 수치 파싱 전에 Rust는 디코딩된 첫 YAML 스칼라 또는 XML 요소 텍스트를 검사하며 이스케이프, CDATA, 밑줄 정규화, YAML 실수의 0 삽입도 포함합니다. YAML 스칼라는 진법별 정수 할당을 제한하기 위해 최대 4096개 16진 숫자 문자를 허용합니다. 보수적인 검사로 네이티브가 수락하는 입력도 거부할 수 있습니다. 소수 자릿수, 계수, 연산 횟수는 [리소스 정책](../resource-policy)을 따릅니다. 이동된 핸들은 읽거나 바꿀 수 없습니다.

## 관련 메서드

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
