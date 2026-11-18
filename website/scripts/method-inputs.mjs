// Build-time AST only: TypeScript is never loaded by the method form.
import ts from 'typescript';
export function methodInputs(source) {
  const file = ts.createSourceFile('example.js', source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  const inputs = [];
  function visit(node) {
    if (ts.isCallExpression(node) && !['assert', 'show', 'print'].includes(node.expression.getText(file))) {
      node.arguments.forEach((arg, index) => {
        const text = arg.getText(file);
        let type, value;
        if (ts.isStringLiteral(arg)) { type = 'string'; value = arg.text; }
        else if (/^-?\d+n$/.test(text)) { type = 'bigint'; value = text.slice(0, -1); }
        else if (/^-?(?:\d+(?:\.\d*)?|\.\d+)(?:e[+-]?\d+)?$/i.test(text)) { type = 'number'; value = text; }
        else if (text === 'true' || text === 'false') { type = 'boolean'; value = text; }
        if (type) inputs.push({ start: arg.getStart(file), end: arg.end, type, value, label: `${node.expression.getText(file)} · ${index + 1}` });
      });
    }
    ts.forEachChild(node, visit);
  }
  visit(file);
  return inputs.sort((a, b) => a.start - b.start);
}
