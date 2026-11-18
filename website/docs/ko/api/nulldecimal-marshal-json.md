---
locale: ko
source_revision: ce07dfd04d96401a00570a69b98dc03abf2e70bda05348ea2e8e6bff0225c443
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::marshal_json"
---
# NullDecimal::marshal_json

NullDecimal을 json 바이트로 인코딩합니다.

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. JSON 바이트를 담은 `Vec<u8>`/Uint8Array를 반환합니다. Rust는 json 기능이 필요하며 브라우저 빌드에서 활성화합니다. 무효인 nullable은 JSON null이며 유효한 값은 내부 Decimal이 미초기화여도 따옴표로 감싼 정규 소수입니다. 반환된 바이트는 독립적인 복사본이며 읽기는 상태를 바꾸지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-json" locale="ko" />

## 브라우저 제한

디코더 입력은 8192바이트까지이며 매개변수 문자열도 Wasm에 복사하기 전에 길이를 검사합니다. Rust는 텍스트와 디코딩된 JSON 문자열에 [소수 리소스 정책](../resource-policy)을 다시 적용하므로 이스케이프로 지수/자릿수 제한을 우회할 수 없습니다. 잘못된 UTF-8과 JSON/소수 문법 진단은 코어가 처리합니다. 입력과 리소스 오류는 수신 객체를 부분적으로 갱신하지 않으며 이동된 래퍼는 읽거나 바꿀 수 없습니다.

## 관련 메서드

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
