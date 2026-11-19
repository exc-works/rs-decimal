// Analyse the fixtures at build time; the browser receives only field metadata.
import ts from 'typescript';
import { readFileSync } from 'node:fs';
import { RoundingMode } from '../src/bridge/index.js';

const declarations = ts.createSourceFile('index.d.ts', readFileSync(new URL('../src/bridge/index.d.ts', import.meta.url), 'utf8'), ts.ScriptTarget.Latest, true);
const parameters = new Map();
function collect(node, owner) {
  const name = node.name?.text;
  const args = node.parameters ?? (node.type && ts.isFunctionTypeNode(node.type) ? node.type.parameters : undefined);
  if (name && args) {
    const names = args.map(p => p.name.text);
    parameters.set(owner + '.' + name, names);
    if (!parameters.has(name)) parameters.set(name, names);
    else if (parameters.get(name)?.join() !== names.join()) parameters.set(name, null);
  }
  ts.forEachChild(node, child => collect(child, name ?? owner));
}
collect(declarations, '');

export function methodInputs(source) {
  const file = ts.createSourceFile('example.js', source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  const inputs = [];
  function visit(node) {
    if (ts.isCallExpression(node) && !['assert', 'show', 'print'].includes(node.expression.getText(file))) {
      const call = node.expression.getText(file);
      const name = call.split('.').at(-1);
      const names = parameters.get(call) ?? parameters.get(name);
      node.arguments.forEach((arg, index) => {
        const text = arg.getText(file);
        let type, value;
        if (ts.isStringLiteral(arg)) { type = 'string'; value = arg.text; }
        else if (/^-?\d+n$/.test(text)) { type = 'bigint'; value = text.slice(0, -1); }
        else if (/^-?(?:\d+(?:\.\d*)?|\.\d+)(?:e[+-]?\d+)?$/i.test(text)) { type = 'number'; value = text; }
        else if (text === 'true' || text === 'false') { type = 'boolean'; value = text; }
        else if (ts.isPropertyAccessExpression(arg) && arg.expression.getText(file) === 'RoundingMode' && Object.hasOwn(RoundingMode, arg.name.text)) {
          type = 'rounding'; value = arg.name.text;
        }
        if (!type) return;
        const variable = node.arguments.length === 1 && ts.isVariableDeclaration(node.parent) ? node.parent.name.getText(file) : undefined;
        const label = type === 'rounding' ? 'mode' : variable && variable !== 'value' ? variable : names?.[index] ?? name;
        inputs.push({ start: arg.getStart(file), end: arg.end, type, value, label });
      });
    }
    ts.forEachChild(node, visit);
  }
  visit(file);
  return inputs.sort((a, b) => a.start - b.start);
}

// Only project edits onto Rust when the entire fixture has a one-to-one token match.
// Native-only operations or substantially different Rust examples keep their original source.
export function rustFields(javascript, rust, fields) {
  function tokens(source, isRust) {
    return [...source.matchAll(/"(?:\\.|[^"\\])*"|[A-Za-z_]\w*|\d+(?:\.\d+)?(?:[eE][+-]?\d+)?n?|::|&&|[^\s]/gu)]
      .filter(m => !isRust || (m[0] !== '?' && m[0] !== '&'))
      .map(m => ({ start: m.index, end: m.index + m[0].length,
        key: m[0] === 'const' ? 'let' : m[0] === '::' ? '.' : m[0].replace(/^(\d+)n$/, '$1') }));
  }
  const js = tokens(javascript, false), rs = tokens(rust, true);
  if (js.length !== rs.length || js.some((t, i) => t.key !== rs[i].key)) return null;
  return fields.map(field => {
    const start = js.findIndex(t => t.start === field.start), end = js.findIndex(t => t.end === field.end);
    return { ...field, start: rs[start].start, end: rs[end].end };
  });
}
