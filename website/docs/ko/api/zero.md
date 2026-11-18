---
locale: ko
source_revision: 751751d0b535375fecff33d2510df43e92ffca892248b1be4dba2a34ce1d0f99
translation_provenance: en-US-korean-reviewed
api_id: zero
---
# zero

소수 자릿수 0인 초기화된 0을 생성합니다.

```rust
fn zero () -> Decimal
```

## 매개변수, 반환값, 사용 시 주의점

매개변수 없음. 새 Decimal을 반환합니다. Decimal::default()와 수치로는 같지만 결과는 초기화되어 있습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-zero" locale="ko" />

## 제한

bridge는 Rust 이름과 매개변수 순서를 유지합니다. 자유 함수는 playground에서 바로 사용할 수 있습니다. 빌린 입력은 소비하거나 바꾸지 않으며 잘못된 핸들이나 다른 실행의 핸들은 거부합니다. 모든 연산은 [리소스 정책](../resource-policy)을 따릅니다.

## 관련 메서드

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
