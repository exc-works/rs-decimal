---
locale: ko
source_revision: c7bf1c667592f8239154dc21841d27e0f4e94230da52fdab553a96aeaa5e48bf
translation_provenance: en-US-korean-reviewed
api_id: Decimal::float32
---
# Decimal::float32

IEEE binary32로 직접 변환하고 정확성을 알립니다.

```rust
fn float32 (& self) -> (f32 , bool)
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. Rust는 (f32, bool)을 반환하며 [number, exact]에 대응합니다. 중간값은 짝수 쪽으로 반올림합니다. 플래그는 변환 성공 여부가 아니라 소수를 정확히 표현했는지를 뜻하므로 0.1은 부정확합니다. 오버플로는 부호 있는 무한대와 false를 반환하고 언더플로는 부호 있는 0을 반환할 수 있습니다. 먼저 binary64로 반올림하지 않고 binary32로 직접 반올림합니다. 예제는 JSON이 특수 값을 숨기지 못하도록 IEEE 비트를 기록합니다. playground print도 NaN, 무한대, 음의 0에 전송된 binary64 비트를 표시합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-float32" locale="ko" />

## 관련 메서드

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
