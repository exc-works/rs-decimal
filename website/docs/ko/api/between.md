---
locale: ko
source_revision: 4488da41f235921047e8f85b8a68b47bc497bf199529326773333d8f22d38585
translation_provenance: en-US-korean-reviewed
api_id: between
---
# between

소수가 양 끝을 포함하는 수치 범위 안에 있는지 확인합니다.

```rust
fn between (v : & Decimal , lower : & Decimal , upper : & Decimal) -> bool
```

## 매개변수, 반환값, 사용 시 주의점

v, lower, upper를 &Decimal로 빌려 lower <= v <= upper의 bool/boolean을 반환합니다. 동등성은 표현 차이를 무시합니다. 경계를 재정렬하지 않으므로 lower > upper이면 false입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-between" locale="ko" />

## 제한

bridge는 Rust 이름과 매개변수 순서를 유지합니다. 자유 함수는 playground에서 바로 사용할 수 있습니다. 빌린 입력은 소비하거나 바꾸지 않으며 잘못된 핸들이나 다른 실행의 핸들은 거부합니다. 모든 연산은 [리소스 정책](../resource-policy)을 따릅니다.

## 관련 메서드

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
