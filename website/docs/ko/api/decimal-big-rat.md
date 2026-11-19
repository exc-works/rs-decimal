---
locale: ko
source_revision: 742bab219fea4dd88e3233aac7a28750ce01392539e76e100af497e171dcc4c9
translation_provenance: en-US-korean-reviewed
api_id: Decimal::big_rat
---
# Decimal::big_rat

정확하게 약분한 유리수 값을 읽습니다.

```rust
fn big_rat (& self) -> BigRational
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. Rust는 BigRational을 반환합니다. JavaScript는 분모가 양수인 정규화된 불변 {numerator: bigint, denominator: bigint} 스냅샷을 반환합니다. 소수 자릿수와 후행 0은 유리수 메타데이터로 유지되지 않으며 0은 0/1이 됩니다. 입력은 빌려 사용하며 바꾸지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-big-rat" locale="ko" />

## 관련 메서드

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
