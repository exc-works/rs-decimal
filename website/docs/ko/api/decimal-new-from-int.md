---
locale: ko
source_revision: aad24e4a42d458d995751545010904db4f3e9453abb0fa3c66514c8c59b55494
translation_provenance: en-US-korean-reviewed
api_id: Decimal::new_from_int
---
# Decimal::new_from_int

네이티브 너비의 부호 있는 정수로 초기화된 정수를 만듭니다.

```rust
fn new_from_int (value : isize) -> Self
```

## 매개변수와 반환값

`value: isize` → 정수 JavaScript `number`. 결과의 소수 자릿수는 0입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-int" locale="ko" />

## 사용 시 주의점과 제한

브라우저 대상은 wasm32이므로 네이티브 Rust가 64비트 환경에서 실행되어도 허용 범위는 -2147483648부터 2147483647까지입니다. 소수, NaN, 무한대는 매개변수 오류입니다.

브라우저는 [리소스 정책](../resource-policy)도 적용합니다. JavaScript 입력은 Rust 호출 전에 검사합니다.

## 관련 메서드

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
