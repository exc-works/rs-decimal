---
locale: ko
source_revision: 0d4a74db932ab126e7566fb630352abba8c528142a3d37c615a2694b7fac9c0a
translation_provenance: en-US-korean-reviewed
api_id: max
---
# max

자유 함수로 두 소수 중 큰 값을 복사합니다.

```rust
fn max (a : & Decimal , b : & Decimal) -> Decimal
```

## 매개변수, 반환값, 사용 시 주의점

a와 b를 &Decimal로 빌려 독립적인 Decimal을 반환합니다. 수치가 같으면 a의 소수 자릿수와 초기화 상태를 유지합니다. Decimal::max와 동일한 라이브러리 동작을 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-max" locale="ko" />

## 제한

bridge는 Rust 이름과 매개변수 순서를 유지합니다. 자유 함수는 playground에서 바로 사용할 수 있습니다. 빌린 입력은 소비하거나 바꾸지 않으며 잘못된 핸들이나 다른 실행의 핸들은 거부합니다. 모든 연산은 [리소스 정책](../resource-policy)을 따릅니다.

## 관련 메서드

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
