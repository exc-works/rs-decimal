---
locale: ko
source_revision: aa764a04d58bbac840fe64bca919cb70175f064a9010f9a840a1f56ffacabc1c
translation_provenance: en-US-korean-reviewed
api_id: Decimal::is_zero
---
# Decimal::is_zero

값에 따라 달라지는 연산 전에 수치가 0인지 확인합니다.

```rust
fn is_zero (& self) -> bool
```

## 매개변수, 반환값, 사용 시 주의점

bool/boolean을 반환하며 미초기화 기본값에도 true를 반환합니다. 초기화 상태가 중요하면 is_nil()을 사용하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-zero" locale="ko" />

## 오류와 제한

이 메서드들은 입력을 빌리며 소비하거나 바꾸지 않습니다. 라이브러리 연산은 Result를 반환하지 않습니다. bridge는 해제되거나 이동된 핸들, 다른 실행의 핸들을 거부하고 [리소스 정책](../resource-policy)을 적용합니다. 예제는 스칼라 출력을 `value` 필드로 감싸며 Decimal 출력에는 전체 스냅샷을 사용합니다.

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
