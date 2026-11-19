---
locale: ko
source_revision: cc346dbc73e3d203c64f61c0e3e68ec08682dec8479c74ee4ea48b4441aef5c2
translation_provenance: en-US-korean-reviewed
api_id: Decimal::new_from_big_int
---
# Decimal::new_from_big_int

임의 크기의 정수를 숫자 손실 없이 소수 자릿수 0인 초기화된 Decimal로 복사합니다.

```rust
fn new_from_big_int (value : & BigInt) -> Self
```

## 매개변수와 반환값

Rust는 `&BigInt`를 빌리고 JavaScript는 불변 `bigint`를 전달합니다. 반환된 계수는 원본과 독립적입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-big-int" locale="ko" />

## 사용 시 참고 사항

이미 정밀도를 잃은 JavaScript number를 나중에 bigint로 바꾸지 마세요. `n`으로 끝나는 정수 리터럴이나 정확한 정수 텍스트에서 시작하세요. playground 입력은 최대 4096자리이며, 네이티브 라이브러리에는 이 playground 전용 제한이 없습니다.

## 관련 메서드

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
