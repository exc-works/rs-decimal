---
locale: ko
source_revision: 00df85dbc78a408d1e3a1357be14942f9ab79794d46b2e7624e5bd2c1ceeaef3
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::unmarshal_param"
---
# Decimal::unmarshal_param

param 입력을 기존 Decimal로 디코딩합니다.

```rust
fn unmarshal_param (& mut self , param : & str) -> Result < () , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

param: &str/string. 수신 객체를 바꾸고 Result<(), DecimalError>/undefined를 반환합니다. 앞뒤 공백 제거와 지수 문법을 포함한 소수 텍스트를 파싱합니다. 빈 텍스트와 리터럴 null은 잘못된 소수 텍스트입니다. 빈 입력이 값 없음을 뜻하면 nullable 디코딩을 사용하세요. null이 아닌 입력을 성공적으로 파싱하면 소수를 초기화하고 0이 아닌 수치의 후행 자릿수를 유지하며 파싱한 0은 소수 자릿수 0으로 정규화합니다. 오류는 수신 객체의 전체 상태를 유지합니다. 라이브러리 디코딩 오류는 Unmarshal과 기저 오류 종류를 유지하며 리소스 실패도 변경 전에 발생합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-param" locale="ko" />

## 브라우저 제한

디코더 입력은 8192바이트까지이며 매개변수 문자열도 Wasm에 복사하기 전에 길이를 검사합니다. Rust는 텍스트와 디코딩된 JSON 문자열에 [소수 리소스 정책](../resource-policy)을 다시 적용하므로 이스케이프로 지수/자릿수 제한을 우회할 수 없습니다. 잘못된 UTF-8과 JSON/소수 문법 진단은 코어가 처리합니다. 입력과 리소스 오류는 수신 객체를 부분적으로 갱신하지 않으며 이동된 래퍼는 읽거나 바꿀 수 없습니다.

## 관련 메서드

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
