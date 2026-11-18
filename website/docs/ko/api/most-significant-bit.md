---
locale: ko
source_revision: cbc68f86b580dd7b41c6b741193a12b44e7d04cdf771d632d510adf82d84c1ba
translation_provenance: en-US-korean-reviewed
api_id: most_significant_bit
---
# most_significant_bit

최상위 1비트의 0부터 시작하는 위치를 구합니다.

```rust
fn most_significant_bit (value : & BigInt) -> Result < usize , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

value: &BigInt를 bigint로 나타냅니다. Result<usize, DecimalError>/number를 반환합니다. 0과 1은 모두 0, 120은 6입니다. 음수 입력은 InvalidArgument입니다. Decimal 메서드가 아니라 자유 함수입니다. 계수를 대상으로 하려면 coefficient()를 전달하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-most-significant-bit" locale="ko" />

## 브라우저 제한

정수 전송은 손실이 있는 JS number를 거치지 않습니다. 변환 튜플은 Rust의 값/플래그 순서를 유지하며 false는 정확한 결과를 보장하지 않습니다. 빌린 입력은 바뀌지 않고 생성자는 결과를 초기화합니다. [리소스 정책](../resource-policy)은 라이브러리 오류와 별개로 정수 입력, 출력 자릿수, 유리수 계수 증가를 제한합니다. Rust 가져오기 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
