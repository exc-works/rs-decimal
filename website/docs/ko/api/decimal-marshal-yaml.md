---
locale: ko
source_revision: 2596ef8441aa7ec726abb87c7af62732ec4f2729da79754eba71b3cb01735596
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::marshal_yaml"
---
# Decimal::marshal_yaml

yaml의 의미상 내용을 얻습니다.

```rust
fn marshal_yaml (& self) -> String
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. String/string을 반환합니다. Rust는 yaml 기능이 필요하며 브라우저에서 활성화되어 있습니다. 직렬화된 YAML 문서가 아니라 의미상 YAML 문자열 스칼라를 반환합니다. 따옴표와 태그가 올바르게 처리되도록 YAML 출력기에 문자열로 전달하세요. 정규 소수 텍스트는 소수 부분의 후행 0을 제거합니다. 기본 미초기화 0은 "0"을 반환합니다. 읽기는 초기화나 유효성을 바꾸지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-yaml" locale="ko" />

## 브라우저 제한

소스는 UTF-8 8192바이트까지입니다. 코어 수치 파싱 전에 Rust는 디코딩된 첫 YAML 스칼라 또는 XML 요소 텍스트를 검사하며 이스케이프, CDATA, 밑줄 정규화, YAML 실수의 0 삽입도 포함합니다. YAML 스칼라는 진법별 정수 할당을 제한하기 위해 최대 4096개 16진 숫자 문자를 허용합니다. 보수적인 검사로 네이티브가 수락하는 입력도 거부할 수 있습니다. 소수 자릿수, 계수, 연산 횟수는 [리소스 정책](../resource-policy)을 따릅니다. 이동된 핸들은 읽거나 바꿀 수 없습니다.

## 관련 메서드

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
