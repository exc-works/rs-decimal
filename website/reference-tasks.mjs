export const taskSymbols = [
  'Decimal::new_from_string',
  'Decimal::round_with_prec',
  'Decimal::rescale',
  'Decimal::string_with_trailing_zeros',
  'Decimal::impl Type < Postgres >',
];
const tasks = {
  'en-US': ['Read an exact amount', 'Round to two decimal places', 'Choose a rounding mode', 'Keep trailing zeros', 'Store a decimal in PostgreSQL'],
  'zh-Hans': ['从字符串读取精确金额', '舍入并保留两位小数', '选择舍入模式', '保留尾零', '将十进制数存入 PostgreSQL'],
  'zh-Hant': ['從字串讀取精確金額', '捨入並保留兩位小數', '選擇捨入模式', '保留尾零', '將十進位數存入 PostgreSQL'],
  fr: ['Lire un montant exact', 'Arrondir à deux décimales', 'Choisir le mode d’arrondi', 'Conserver les zéros finaux', 'Stocker une valeur dans PostgreSQL'],
  de: ['Einen exakten Betrag einlesen', 'Auf zwei Nachkommastellen runden', 'Rundungsmodus wählen', 'Nachgestellte Nullen erhalten', 'Dezimalzahlen in PostgreSQL speichern'],
  ja: ['文字列から正確な金額を読み取る', '小数第2位に丸める', '丸めモードを選ぶ', '末尾のゼロを保持する', 'PostgreSQL に保存する'],
  ko: ['문자열에서 정확한 금액 읽기', '소수점 둘째 자리로 반올림', '반올림 모드 선택', '끝자리 0 유지', 'PostgreSQL에 십진수 저장'],
};
tasks['en-GB'] = tasks['en-US'];
export function taskLinks(locale) {
  return taskSymbols.map((id, index) => ({ id, title: tasks[locale][index] }));
}
