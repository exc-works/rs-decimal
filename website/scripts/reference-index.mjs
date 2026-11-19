import { readFileSync } from 'node:fs';

const read = path => readFileSync(new URL(path, import.meta.url), 'utf8');
const inventory = JSON.parse(read('../api/rust-api.json'));
const coverage = JSON.parse(read('../api/coverage.json'));
const containers = new Set(inventory.filter(item => ['struct', 'enum', 'module'].includes(item.kind)).map(item => item.id));

const decimalGroups = {
  construct: 'must_from_string try_from_coefficient',
  compare: 'cmp equal gt gte has_fraction is_integer is_negative is_nil is_not_zero is_positive is_zero lt lte max max_ref min min_ref not_equal sign',
  precision: 'ceil ceil_with_prec floor floor_with_prec into_rescaled precision rescale rescale_down round round_with_prec scale significant_figures strip_trailing_zeros truncate truncate_with_prec',
  convert: 'append_text append_text_with_trailing_zeros big_int big_rat bit_len coefficient float32 float64 format_go format_with_separators int64 int_part into_coefficient scan string string_with_trailing_zeros uint64 value',

};
const arithmetic = {
  addition: 'add safe_add add_raw add_owned',
  subtraction: 'sub safe_sub sub_raw sub_owned',
  multiplication: 'mul_exact mul mul_with_prec mul_down',
  division: 'quo quo_with_prec quo_down quo_rem',
  remainders: 'remainder modulo r#mod',
  powers: 'power sqrt sqrt_with_prec approx_root approx_root_with_prec',
  logs: 'ln ln_with_prec log2 log2_with_prec log10 log10_with_prec exp exp_with_prec',
  signs: 'abs neg must_non_negative shift',
};
decimalGroups.arithmetic = Object.values(arithmetic).join(' ');
const methodGroup = new Map(Object.entries(decimalGroups).flatMap(([group, names]) =>
  names.split(' ').map(name => [name, group])));

function parentOf(id) {
  let prefix = id;
  while (prefix.includes('::') || prefix.includes('.')) {
    prefix = prefix.replace(/(?:::|\.)[^:.]*$/, '');
    if (containers.has(prefix)) return prefix;
  }
  return null;
}

function groupOf(item, parent) {
  if (item.kind === 'reexport') return 'exports';
  if (['derived_trait','trait_impl','trait_method','macro_trait','associated_type'].includes(item.kind)) return 'traits';
  if (parent === 'Decimal' && item.kind === 'method') {
    const name = item.id.slice('Decimal::'.length);
    if (name.startsWith('new')) return 'construct';
    if (/^(un)?marshal_/.test(name)) return 'convert';
    return methodGroup.get(name) ?? 'members';
  }
  if (item.kind === 'field') return 'fields';
  if (item.kind === 'variant') return 'variants';
  if (containers.has(item.id)) return 'types';
  return parent ? 'members' : 'functions';
}

const byId = new Map(coverage.map(item => [item.id, item]));
export const symbols = inventory.map(item => {
  const { doc_id: slug, disposition } = byId.get(item.id);
  const parent = parentOf(item.id);
  return { ...item, slug, disposition, parent, group: groupOf(item, parent) };
});
export const symbolById = new Map(symbols.map(item => [item.id, item]));
export function apiPath(locale, item) {
  return '/' + locale + '/api/' + item.slug + '.html';
}

const summaries = new Map();
export function summary(locale, symbol) {
  const key = apiPath(locale, symbol);
  if (!summaries.has(key)) {
    const page = read('../docs/' + locale + '/api/' + symbol.slug + '.md');
    const paragraph = page.match(/^# [^\n]+\n\s*\n([^\n]+)/m)?.[1];
    if (!paragraph) throw Error('Missing API summary: ' + key);
    summaries.set(key, paragraph);
  }
  return summaries.get(key);
}

// Keep common arithmetic in reading order; derive the other families from the index.
export const operationGroups = [
  ...Object.entries(arithmetic).map(([key, names]) => ({
    key, items: names.split(' ').map(name => symbolById.get('Decimal::' + name)),
  })),
  ...['construct', 'compare', 'precision', 'convert'].map(key => ({
    key, items: symbols.filter(item => item.parent === 'Decimal' && item.group === key),
  })),
];
