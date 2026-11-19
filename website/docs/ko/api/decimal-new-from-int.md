---
locale: ko
source_revision: dd3a6777ec0d2daf335730b8ca67dd44bd500850bf5f5a894aa8a18b1d5c40f9
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

## 사용 시 참고 사항

브라우저 대상은 wasm32이므로 네이티브 Rust가 64비트 환경에서 실행되어도 허용 범위는 -2147483648부터 2147483647까지입니다. 소수, NaN, 무한대는 매개변수 오류입니다.

## 관련 메서드

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
