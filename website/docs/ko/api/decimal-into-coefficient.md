---
locale: ko
source_revision: cc52467a55bcaa7e86e52d9cf523d5b8a22924e126619052d59e51cd0b6aab8b
translation_provenance: en-US-korean-reviewed
api_id: Decimal::into_coefficient
---
# Decimal::into_coefficient

스케일 적용 전 정수 계수를 이동하여 꺼냅니다.

```rust
fn into_coefficient (self) -> BigInt
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. Rust에서는 BigInt, JS에서는 bigint를 반환합니다. 소수 자릿수와 초기화 상태는 사라지므로 1.2300은 12300이 됩니다. 기본 0은 0입니다. Decimal은 소비되어 다시 읽을 수 없습니다. JavaScript에서 소비하지 않고 스냅샷을 얻으려면 coefficient를 사용하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-coefficient" locale="ko" />

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
