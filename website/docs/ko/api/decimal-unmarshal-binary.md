---
locale: ko
source_revision: 8a589312845f2d23d85f3d6da1987ee87f79dfecdc97a705386e9a336b17d7ed
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::unmarshal_binary"
---
# Decimal::unmarshal_binary

Go 호환 바이너리 스칼라로 소수를 교체합니다.

```rust
fn unmarshal_binary (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

bytes: &[u8]/Uint8Array. 수신 객체를 바꾸고 `Result<(), DecimalError>`/undefined를 반환합니다. 첫 4바이트는 빅엔디언 소수 자릿수이며 Gob 헤더 2(양수) 또는 3(음수)와 절댓값이 이어집니다. 성공하면 인코딩된 자릿수를 유지합니다. 빈 입력은 기본 미초기화 0과 달리 초기화된 0을 만듭니다. Gob 페이로드가 없는 4바이트 접두부도 해당 자릿수의 초기화된 0으로 수락합니다. 짧은 접두부, 잘못된 버전, 131072를 넘는 자릿수는 코어 Unmarshal 오류를 반환합니다. 오류는 수신 객체를 유지합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-binary" locale="ko" />

## 관련 메서드

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
