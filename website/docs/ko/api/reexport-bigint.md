---
locale: ko
source_revision: 8110d2fd30808fd737c3f5730a5be2d19cf89f407f5d688859de476689342458
translation_provenance: en-US-korean-reviewed
api_id: reexport::BigInt
---
# reexport::BigInt

크레이트 루트에서 BigInt를 가져옵니다.

```rust
pub use num_bigint::BigInt as BigInt
```

## 매개변수, 반환값, 일반적인 사용법

공개 Rust 재내보내기이며 새 타입이나 구현이 아닙니다. 가져오기 문법은 네이티브에서 검사합니다. 브라우저에서는 각각 bigint, 정확한 유리수 필드 객체, 전역 most_significant_bit 함수에 대응합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-bigint" locale="ko" />

## 브라우저 제한

정수 전송은 손실이 있는 JS number를 거치지 않습니다. 변환 튜플은 Rust의 값/플래그 순서를 유지하며 false는 정확한 결과를 보장하지 않습니다. 빌린 입력은 바뀌지 않고 생성자는 결과를 초기화합니다. [리소스 정책](../resource-policy)은 라이브러리 오류와 별개로 정수 입력, 출력 자릿수, 유리수 계수 증가를 제한합니다. Rust 가져오기 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
