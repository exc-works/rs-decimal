import { sourceValid } from '../runner/protocol.js';
import { RoundingMode } from '../bridge/index.js';

function literal(field, value, language) {
  if (typeof value !== 'string' || value.length > 65536) throw Error('Invalid parameter');
  const rust = language === 'rust';
  if (field.type === 'string') {
    const quoted = JSON.stringify(value);
    if (!rust) return quoted;
    if (/[\uD800-\uDFFF]/u.test(value)) throw Error('Rust strings require Unicode scalar values');
    return quoted.replace(/\\(?:[\\"/bfnrt]|u[\da-f]{4})/gi, escape => {
      if (escape === '\\b') return '\\u{8}';
      if (escape === '\\f') return '\\u{c}';
      if (escape.startsWith('\\u')) return '\\u{' + escape.slice(2) + '}';
      return escape;
    });
  }
  if (field.type === 'rounding' && Object.hasOwn(RoundingMode, value)) return 'RoundingMode' + (rust ? '::' : '.') + value;
  if (field.type === 'bigint' && /^-?(?:0|[1-9]\d*)$/.test(value)) return value + (rust ? '' : 'n');
  if (field.type === 'number' && /^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:e[+-]?\d+)?$/i.test(value) && Number.isFinite(Number(value))) {
    if (!rust) return value;
    const number = Number(value);
    if (field.integer) {
      if (!Number.isSafeInteger(number)) throw Error('Not a Rust integer');
      return String(number);
    }
    return /[.e]/i.test(value) ? value : value + '.0';
  }
  if (field.type === 'boolean' && ['true', 'false'].includes(value)) return value;
  throw Error('Invalid parameter');
}

export function previewParts(source, fields, values, language) {
  if (!Array.isArray(values) || values.length !== fields.length) throw Error('Invalid parameters');
  const parts = [];
  let offset = 0;
  for (const [i, field] of fields.entries()) {
    parts.push({ text: source.slice(offset, field.start) });
    const changed = values[i] !== field.value;
    parts.push({ text: changed ? literal(field, values[i], language) : source.slice(field.start, field.end), changed });
    offset = field.end;
  }
  parts.push({ text: source.slice(offset) });
  return parts;
}
export function formSource(source, fields, values) {
  const result = previewParts(source, fields, values, 'javascript').map(part => part.text).join('') + '\nconsole.log(value);\n';
  if (!sourceValid(result)) throw Error('Example too large');
  return result;
}
