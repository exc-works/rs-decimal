---
locale: ko
source_revision: b417c8ff7bf7c7086d2d314348cd490ed506bea3f1d87b65cb817d90defb82d7
translation_provenance: en-US-korean-reviewed
api_id: Decimal::big_rat
---
# Decimal::big_rat

정확하게 약분한 유리수 값을 읽습니다.

```rust
fn big_rat (& self) -> BigRational
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. Rust는 BigRational을 반환합니다. bridge는 분모가 양수인 정규화된 불변 {numerator: bigint, denominator: bigint} 스냅샷을 반환합니다. 소수 자릿수와 후행 0은 유리수 메타데이터로 유지되지 않으며 0은 0/1이 됩니다. 입력은 빌려 사용하며 바꾸지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-big-rat" locale="ko" />

## 브라우저 제한

정수 전송은 손실이 있는 JS number를 거치지 않습니다. 변환 튜플은 Rust의 값/플래그 순서를 유지하며 false는 정확한 결과를 보장하지 않습니다. 빌린 입력은 바뀌지 않고 생성자는 결과를 초기화합니다. [리소스 정책](../resource-policy)은 라이브러리 오류와 별개로 정수 입력, 출력 자릿수, 유리수 계수 증가를 제한합니다. Rust 가져오기 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
