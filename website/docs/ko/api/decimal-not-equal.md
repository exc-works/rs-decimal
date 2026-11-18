---
locale: ko
source_revision: 7f94821cf570d2860956d25bb69a5d1cdbe5a253fac837b02903b87236f85c19
translation_provenance: en-US-korean-reviewed
api_id: Decimal::not_equal
---
# Decimal::not_equal

두 소수가 서로 다른 수치를 나타내는지 확인합니다.

```rust
fn not_equal (& self , other : & Self) -> bool
```

## 매개변수, 반환값, 사용 시 주의점

other: &Decimal을 빌려 bool/boolean을 반환합니다. equal()의 반대이며 계수나 소수 자릿수 스냅샷을 비교하는 것이 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-not-equal" locale="ko" />

## 오류와 제한

이 메서드들은 입력을 빌리며 소비하거나 바꾸지 않습니다. 라이브러리 연산은 Result를 반환하지 않습니다. bridge는 해제되거나 이동된 핸들, 다른 실행의 핸들을 거부하고 [리소스 정책](../resource-policy)을 적용합니다. 예제는 스칼라 출력을 `value` 필드로 감싸며 Decimal 출력에는 전체 스냅샷을 사용합니다.

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
