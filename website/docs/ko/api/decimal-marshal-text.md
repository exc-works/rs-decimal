---
locale: ko
source_revision: 38a30bb0b4a307093eafd823ad26d297985a36153b191d7c695a0f68960ea0e6
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::marshal_text"
---
# Decimal::marshal_text

Decimal을 text 바이트로 인코딩합니다.

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. 정규 ASCII 소수 텍스트를 담은 `Vec<u8>`/Uint8Array를 반환합니다. 소수 부분의 후행 0을 제거하며 기본 Decimal 0은 ASCII 0을 출력합니다. 반환된 바이트는 독립적인 복사본이며 읽기는 상태를 바꾸지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-text" locale="ko" />

## 브라우저 제한

디코더 입력은 8192바이트까지이며 매개변수 문자열도 Wasm에 복사하기 전에 길이를 검사합니다. Rust는 텍스트와 디코딩된 JSON 문자열에 [소수 리소스 정책](../resource-policy)을 다시 적용하므로 이스케이프로 지수/자릿수 제한을 우회할 수 없습니다. 잘못된 UTF-8과 JSON/소수 문법 진단은 코어가 처리합니다. 입력과 리소스 오류는 수신 객체를 부분적으로 갱신하지 않으며 이동된 래퍼는 읽거나 바꿀 수 없습니다.

## 관련 메서드

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
