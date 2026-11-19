---
locale: ko
source_revision: d578df6b6b71b8ce4b0e165f6e630705dc946bcefab55f8b43fc9cba6d60bb70
translation_provenance: en-US-korean-reviewed
api_id: Decimal::new
---
# Decimal::new

초기화된 정수를 만듭니다. 개수나 정수 금액 등 부호 있는 64비트 정수에 이 생성자를 사용하세요.

```rust
fn new (value : i64) -> Self
```

## 매개변수와 반환값

`value: i64` → JavaScript `bigint`. 결과의 소수 자릿수는 0입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new" locale="ko" />

## 사용 시 참고 사항

JavaScript number 대신 `12n`을 사용하세요. bridge는 i64 범위 밖의 값을 거부합니다. 소수 값에는 `new_from_string`을 사용하세요.

## 관련 메서드

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
