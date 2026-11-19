---
locale: ko
source_revision: 284f92d20227e82946bbbcda123bebd9143e64512e1a7f0633cf17755157f389
---
# Playground

실행을 눌러 단가 4.50인 공책 두 권에 배송비 1.25를 더해 보세요. 결제 금액은 10.25이고 20.00을 내면 거스름돈은 9.75입니다. 수량을 `2n`에서 `3n`으로 바꾸고 다시 실행하면 결제 금액은 14.75가 됩니다.

작성한 초안이 열려 있다면 먼저 내보내서 저장한 다음 **예제 초기화**를 눌러 영수증을 불러오세요.

<script setup>
import Playground from "../_components/Playground.vue";
import { locales } from "../../locales.mjs";
</script>

<Playground locale="ko" :labels="locales['ko']" />

::: details 메서드 예제와 공유
메서드 페이지에서 인수를 바꾸면 코드와 예상 결과가 갱신됩니다. 전체 코드를 편집하거나 공유하려면 Playground에서 여세요.

공유 기능은 최대 4 KiB의 UTF-8 코드와 라이브러리 빌드 커밋을 URL 프래그먼트에 저장합니다. 코드는 서버로 업로드되지 않습니다. 공유 링크는 동일한 빌드가 필요하며, 코드를 실행 없이 불러오고 이전 결과를 지웁니다. 잘못된 링크는 현재 초안을 유지합니다. 코드를 확인한 후 실행하세요. 더 큰 스크립트는 파일로 내보내세요. 편집기는 최대 64 KiB를 지원합니다. 링크에는 스크립트의 텍스트가 포함되므로 공유하기 전에 코드를 확인하세요.
:::

[Playground 리소스 정책](./resource-policy)
