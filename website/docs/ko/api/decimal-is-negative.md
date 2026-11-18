---
locale: ko
source_revision: 9ffad200b8eee0be119e53ec6189311fb6b54cada31774d04149bdc2b868ac4e
translation_provenance: en-US-korean-reviewed
api_id: Decimal::is_negative
---
# Decimal::is_negative

수치가 엄격히 음수인지 확인합니다.

```rust
fn is_negative (& self) -> bool
```

## 매개변수, 반환값, 사용 시 주의점

bool/boolean을 반환합니다. 미초기화 기본값을 포함한 0에는 false를 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-negative" locale="ko" />

## 오류와 제한

이 메서드들은 입력을 빌리며 소비하거나 바꾸지 않습니다. 라이브러리 연산은 Result를 반환하지 않습니다. bridge는 해제되거나 이동된 핸들, 다른 실행의 핸들을 거부하고 [리소스 정책](../resource-policy)을 적용합니다. 예제는 스칼라 출력을 `value` 필드로 감싸며 Decimal 출력에는 전체 스냅샷을 사용합니다.

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
