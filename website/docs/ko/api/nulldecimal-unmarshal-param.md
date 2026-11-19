---
locale: ko
source_revision: 7c807773ded0adcdcd91027cab2d90a4029760f6cf1ca4fd49648d1e294b17d6
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::unmarshal_param"
---
# NullDecimal::unmarshal_param

param 입력을 기존 NullDecimal로 디코딩합니다.

```rust
fn unmarshal_param (& mut self , param : & str) -> Result < () , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

param: &str/string. 수신 객체를 바꾸고 Result<(), DecimalError>/undefined를 반환합니다. 앞뒤 공백 제거와 지수 문법을 포함한 소수 텍스트를 파싱합니다. 빈 입력은 유효성을 해제하고 내부 Decimal을 재설정합니다. 리터럴 null은 잘못된 소수 텍스트입니다. null이 아닌 입력을 성공적으로 파싱하면 소수를 초기화하고 0이 아닌 수치의 후행 자릿수를 유지하며 파싱한 0은 소수 자릿수 0으로 정규화합니다. valid=true로 설정합니다. 오류는 수신 객체의 전체 상태를 유지합니다. 라이브러리 디코딩 오류는 Unmarshal과 기저 오류 종류를 유지하며 리소스 실패도 변경 전에 발생합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-unmarshal-param" locale="ko" />

## 관련 메서드

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
