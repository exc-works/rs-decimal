---
locale: ko
source_revision: 8ab6c44a0e4cf327b955869493409ea9bcd447a1bc1451590926853f00b0ca0b
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::unmarshal_json"
---
# Decimal::unmarshal_json

json 입력을 기존 Decimal로 디코딩합니다.

```rust
fn unmarshal_json (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

bytes: &[u8]/Uint8Array. 수신 객체를 바꾸고 Result<(), DecimalError>/undefined를 반환합니다. Rust는 json 기능이 필요합니다. 원래 UTF-8 JSON 바이트를 전달하세요. 숫자는 부동소수점 왕복 없이 검사하고 문자열은 이스케이프 해제 후 정확한 소수 파싱을 합니다. 먼저 JSON.parse로 숫자 토큰을 JS number로 바꾸지 마세요. JSON 공백은 space/tab/CR/LF뿐입니다. JSON null은 초기화를 포함한 기존 Decimal 전체를 바꾸지 않습니다. null이 아닌 입력을 성공적으로 파싱하면 소수를 초기화하고 0이 아닌 수치의 후행 자릿수를 유지하며 파싱한 0은 소수 자릿수 0으로 정규화합니다. 오류는 수신 객체의 전체 상태를 유지합니다. 라이브러리 디코딩 오류는 Unmarshal과 기저 오류 종류를 유지하며 리소스 실패도 변경 전에 발생합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-json" locale="ko" />

## 브라우저 제한

디코더 입력은 8192바이트까지이며 매개변수 문자열도 Wasm에 복사하기 전에 길이를 검사합니다. Rust는 텍스트와 디코딩된 JSON 문자열에 [소수 리소스 정책](../resource-policy)을 다시 적용하므로 이스케이프로 지수/자릿수 제한을 우회할 수 없습니다. 잘못된 UTF-8과 JSON/소수 문법 진단은 코어가 처리합니다. 입력과 리소스 오류는 수신 객체를 부분적으로 갱신하지 않으며 이동된 래퍼는 읽거나 바꿀 수 없습니다.

## 관련 메서드

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
