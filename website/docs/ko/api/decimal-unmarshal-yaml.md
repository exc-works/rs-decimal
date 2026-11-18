---
locale: ko
source_revision: 8604d7a6ce89905f3e2b134c8e0e9feabf234b540c25108481cbbaa1614462a5
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::unmarshal_yaml"
---
# Decimal::unmarshal_yaml

yaml을 기존 값으로 디코딩합니다.

```rust
fn unmarshal_yaml (& mut self , source : & str) -> Result < () , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

source: &str/string. 수신 객체를 바꾸고 `Result<(), DecimalError>`/undefined를 반환합니다. Rust는 yaml이 필요합니다. Rust가 따옴표, 태그, 정확한 숫자를 사용할 수 있도록 원래 YAML 소스를 전달하세요. 따옴표 없는 012는 8진수로 해석하여 10이며 따옴표 있는 "012" 또는 !!str 012는 십진수 12입니다. 숫자 태그/진법/밑줄은 코어 해석기를 따릅니다. 따옴표 없는 1_000은 1000이지만 "1_000"은 잘못된 값입니다. 불리언, 컬렉션, 블록 스칼라는 거부합니다. 첫 문서만 값을 제공하지만 이후 문서도 문법은 유효해야 합니다. Null 또는 빈 문서는 초기화와 소수 자릿수를 포함한 기존 수신 객체를 바꾸지 않습니다. null이 아닌 입력의 디코딩이 성공하면 소수를 초기화하고 0이 아닌 수치의 후행 자릿수를 유지합니다. 오류는 수신 객체의 모든 필드를 유지하고 Unmarshal을 포함한 라이브러리 오류 종류를 유지합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-unmarshal-yaml" locale="ko" />

## 브라우저 제한

소스는 UTF-8 8192바이트까지입니다. 코어 수치 파싱 전에 Rust는 디코딩된 첫 YAML 스칼라 또는 XML 요소 텍스트를 검사하며 이스케이프, CDATA, 밑줄 정규화, YAML 실수의 0 삽입도 포함합니다. YAML 스칼라는 진법별 정수 할당을 제한하기 위해 최대 4096개 16진 숫자 문자를 허용합니다. 보수적인 검사로 네이티브가 수락하는 입력도 거부할 수 있습니다. 소수 자릿수, 계수, 연산 횟수는 [리소스 정책](../resource-policy)을 따릅니다. 이동된 핸들은 읽거나 바꿀 수 없습니다.

## 관련 메서드

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
