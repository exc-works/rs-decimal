// Keys are shared by the task directory, the type navigation and the catalogue.
const keys = 'types construct compare arithmetic precision convert fields variants traits members functions exports all result example'.split(' ');
const labels = {
  'en-US': 'Types and modules|Construction and parsing|Comparison|Arithmetic|Precision and rounding|Formatting and conversion|Fields|Variants|Trait implementations|Members|Functions and constants|Re-exports|All symbols|Result|Example',
  'zh-Hans': '类型与模块|构造与解析|比较|算术|精度与舍入|格式化与转换|字段|枚举值|Trait 实现|成员|函数与常量|重新导出|全部符号|结果|示例',
  'zh-Hant': '型別與模組|建構與解析|比較|算術|精度與捨入|格式化與轉換|欄位|列舉值|Trait 實作|成員|函式與常數|重新匯出|全部符號|結果|範例',
  fr: 'Types et modules|Construction et analyse|Comparaison|Arithmétique|Précision et arrondi|Formatage et conversion|Champs|Variantes|Implémentations de traits|Membres|Fonctions et constantes|Réexportations|Tous les symboles|Résultat|Exemple',
  de: 'Typen und Module|Erstellung und Einlesen|Vergleich|Arithmetik|Genauigkeit und Rundung|Formatierung und Konvertierung|Felder|Varianten|Trait-Implementierungen|Elemente|Funktionen und Konstanten|Reexporte|Alle Symbole|Ergebnis|Beispiel',
  ja: '型とモジュール|生成と解析|比較|算術|精度と丸め|書式と変換|フィールド|列挙子|トレイト実装|メンバー|関数と定数|再エクスポート|すべてのシンボル|結果|使用例',
  ko: '타입과 모듈|생성과 파싱|비교|산술|정밀도와 반올림|서식과 변환|필드|열거형 값|트레이트 구현|멤버|함수와 상수|다시 내보내기|전체 심볼|결과|예제',
};
labels['en-GB'] = labels['en-US'];
export const referenceLabels = Object.fromEntries(Object.entries(labels).map(([lang, row]) =>
  [lang, Object.fromEntries(row.split('|').map((label, index) => [keys[index], label]))]));

const operations = {
  'en-US': ['Browse by operation', 'Addition', 'Subtraction', 'Multiplication', 'Division', 'Remainders', 'Powers and roots', 'Logarithms and exponentials', 'Sign and decimal shifts'],
  'zh-Hans': ['按操作查找', '加法', '减法', '乘法', '除法', '取余', '乘方与开方', '对数与指数', '符号与小数点移动'],
  'zh-Hant': ['依操作查找', '加法', '減法', '乘法', '除法', '取餘數', '乘方與開根號', '對數與指數', '正負號與小數點移位'],
  fr: ['Par opération', 'Addition', 'Soustraction', 'Multiplication', 'Division', 'Restes', 'Puissances et racines', 'Logarithmes et exponentielles', 'Signe et décalage décimal'],
  de: ['Nach Rechenoperation', 'Addition', 'Subtraktion', 'Multiplikation', 'Division', 'Restberechnung', 'Potenzen und Wurzeln', 'Logarithmen und Exponentialfunktion', 'Vorzeichen und Dezimalverschiebung'],
  ja: ['操作から探す', '加算', '減算', '乗算', '除算', '剰余', '累乗と根', '対数と指数', '符号と小数点の移動'],
  ko: ['연산별로 찾기', '덧셈', '뺄셈', '곱셈', '나눗셈', '나머지', '거듭제곱과 근', '로그와 지수', '부호와 소수점 이동'],
};
operations['en-GB'] = operations['en-US'];
const operationKeys = ['operations', 'addition', 'subtraction', 'multiplication', 'division', 'remainders', 'powers', 'logs', 'signs'];
for (const [lang, values] of Object.entries(operations)) {
  operationKeys.forEach((key, index) => { referenceLabels[lang][key] = values[index]; });
}
