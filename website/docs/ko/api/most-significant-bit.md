---
locale: ko
source_revision: e0008f3df5627ec04d695568de0e59f79f0a5617252c018a2798ad7b622bab4d
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

## 관련 메서드

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
