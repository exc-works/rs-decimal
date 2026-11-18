---
locale: ko
source_revision: 1cd08b3844b9339373458bc008707757ca17e7772da4c888bc88a31f9efef788
translation_provenance: en-US-korean-reviewed
api_id: Decimal::min
---
# Decimal::min

두 수치 중 작은 값을 복사합니다.

```rust
fn min (& self , other : & Self) -> Self
```

## 매개변수, 반환값, 사용 시 주의점

other: &Decimal을 빌려 독립적인 Decimal을 반환합니다. 수치가 같으면 첫 피연산자의 소수 자릿수와 초기화 상태를 유지합니다. Rust에서는 Decimal::min(&left, &right)를 사용하여 값을 소비하는 Ord::min 트레이트 메서드가 선택되지 않도록 하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-min" locale="ko" />

## 오류와 제한

이 메서드들은 입력을 빌리며 소비하거나 바꾸지 않습니다. 라이브러리 연산은 Result를 반환하지 않습니다. bridge는 해제되거나 이동된 핸들, 다른 실행의 핸들을 거부하고 [리소스 정책](../resource-policy)을 적용합니다. 예제는 스칼라 출력을 `value` 필드로 감싸며 Decimal 출력에는 전체 스냅샷을 사용합니다.

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
