---
locale: ko
source_revision: 753b858d717076ef46e98ae0c0b43697fb31b0651286396a37e7378758f67207
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::string"
---
# NullDecimal::string

nullable의 정규 텍스트를 읽습니다.

```rust
fn string (& self) -> String
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. String/string을 반환합니다. 유효한 값은 내부 Decimal의 정규 텍스트를 사용하여 소수 부분의 후행 0을 제거합니다. 무효인 값은 숨겨진 내부 데이터와 관계없이 리터럴 null을 반환합니다. 무효인 값에 빈 바이트를 반환하는 marshal_text와 다릅니다. 읽기는 유효성이나 초기화 상태를 바꾸지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-string" locale="ko" />

## 브라우저 제한

디코더 입력은 8192바이트까지이며 매개변수 문자열도 Wasm에 복사하기 전에 길이를 검사합니다. Rust는 텍스트와 디코딩된 JSON 문자열에 [소수 리소스 정책](../resource-policy)을 다시 적용하므로 이스케이프로 지수/자릿수 제한을 우회할 수 없습니다. 잘못된 UTF-8과 JSON/소수 문법 진단은 코어가 처리합니다. 입력과 리소스 오류는 수신 객체를 부분적으로 갱신하지 않으며 이동된 래퍼는 읽거나 바꿀 수 없습니다.

## 관련 메서드

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
