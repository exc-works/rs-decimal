---
locale: ko
source_revision: 50a6a7b47a0234b339cf6d517b9928ec7d1775457e7eacbe40da86286996212a
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::unmarshal_bson_value"
---
# Decimal::unmarshal_bson_value

BSON 스칼라를 기존 값으로 디코딩합니다.

```rust
fn unmarshal_bson_value (& mut self , kind : u8 , data : & [u8]) -> Result < () , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

kind: u8/number, data: &[u8]/Uint8Array. 수신 객체를 바꾸고 `Result<(), DecimalError>`/undefined를 반환합니다. Rust는 bson이 필요합니다. double (1), string (2), null (10), int32 (16), int64 (18), Decimal128 (19)를 지원하며 숫자 페이로드는 BSON 리틀엔디언 바이트를 사용합니다. int64에는 JS number 정밀도 손실을 피하도록 원시 바이트를 전달하세요. BSON null은 수신 객체를 미초기화 0으로 재설정하며, 변경하지 않는 unmarshal_json(null)과 다릅니다. null이 아닌 입력의 nullable 디코딩이 성공하면 valid=true로 설정합니다. 미지원 종류, 잘린 데이터, 비유한 숫자는 부분 변경 없이 실패합니다. 호환성 주의점으로 문자열 디코딩은 선언된 마지막 바이트를 NUL인지 확인하지 않고 건너뛰며 필요한 페이로드 이후 바이트는 무시합니다. Decimal128은 Rust 코어가 디코딩하며 지원하는 소수 정밀도를 유지하지만 marshal_bson_value 출력은 여전히 문자열입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-bson-value" locale="ko" />

## 관련 메서드

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
