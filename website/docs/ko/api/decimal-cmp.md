---
locale: ko
source_revision: 19a39638a4734aeba49de6f5af4a9f79e89b842f7b78ba2cf7aeb511f0ebcd6c
translation_provenance: en-US-korean-reviewed
api_id: Decimal::cmp
---
# Decimal::cmp

두 수치를 비교하여 세 가지 결과 중 하나를 반환합니다.

```rust
fn cmp (& self , other : & Self) -> i32
```

## 매개변수, 반환값, 사용 시 주의점

other: &Decimal(JavaScript에서는 Decimal)을 빌립니다. i32/number인 -1, 0, 1을 반환합니다. 소수 자릿수나 초기화 상태가 달라도 수치의 동등성에는 영향이 없습니다. Ordering을 반환하는 Ord::cmp가 아니라 고유 메서드입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-cmp" locale="ko" />

## 오류와 제한

이 메서드들은 입력을 빌리며 소비하거나 바꾸지 않습니다. 라이브러리 연산은 Result를 반환하지 않습니다. bridge는 해제되거나 이동된 핸들, 다른 실행의 핸들을 거부하고 [리소스 정책](../resource-policy)을 적용합니다. 예제는 스칼라 출력을 `value` 필드로 감싸며 Decimal 출력에는 전체 스냅샷을 사용합니다.

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
