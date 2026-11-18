import { sourceValid } from '../runner/protocol.js';
export function formSource(source, fields, values) {
  if (values.length !== fields.length) throw Error('Invalid parameters');
  let result = source;
  for (let i = fields.length - 1; i >= 0; i--) {
    const field = fields[i], value = values[i];
    if (typeof value !== 'string' || value.length > 65536) throw Error('Invalid parameter');
    let literal;
    if (field.type === 'string') literal = JSON.stringify(value);
    else if (field.type === 'bigint' && /^-?(?:0|[1-9]\d*)$/.test(value)) literal = value + 'n';
    else if (field.type === 'number' && /^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:e[+-]?\d+)?$/i.test(value) && Number.isFinite(Number(value))) literal = value;
    else if (field.type === 'boolean' && ['true', 'false'].includes(value)) literal = value;
    else throw Error('Invalid parameter');
    result = result.slice(0, field.start) + literal + result.slice(field.end);
  }
  result += '\nshow(value);\n';
  if (!sourceValid(result)) throw Error('Example too large');
  return result;
}
