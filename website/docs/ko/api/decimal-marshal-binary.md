---
locale: ko
source_revision: aa67b8e6e6f3fb7b6a66f11573d9907fb9dc057abd9547e4d0c0474969528084
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::marshal_binary"
---
# Decimal::marshal_binary

Go 호환 precision/Gob 전송 형식으로 인코딩합니다.

```rust
fn marshal_binary (& self) -> Result < Vec < u8 > , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. `Result<Vec<u8>, DecimalError>`/Uint8Array를 반환합니다. 초기화된 값은 소수 부분의 후행 0을 제거한 뒤 4바이트 빅엔디언 소수 자릿수, big.Int Gob 버전/부호 바이트, 빅엔디언 절댓값을 씁니다. 소수 스칼라 페이로드이며 IEEE 실수가 아닙니다. 미초기화 0은 빈 바이트, 초기화된 0은 [0,0,0,0,2]입니다. 반환된 복사본을 바꿔도 Decimal은 변하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-binary" locale="ko" />

## 관련 메서드

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
