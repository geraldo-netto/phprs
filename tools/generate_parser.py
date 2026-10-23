#!/usr/bin/env python3
# Copyright 2026 Geraldo Netto
# SPDX-License-Identifier: Apache-2.0

"""Regenerate Rust LALR tables using only Python and installed GNU Bison."""
import ast
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / 'grammar/independent'
VOCABULARY = ROOT / 'grammar/vocabulary.json'
COPYRIGHT = 'Copyright 2026 Geraldo Netto'
RUST_HEADER = f'// {COPYRIGHT}\n// SPDX-License-Identifier: Apache-2.0\n\n'
GRAMMAR_HEADER = f'/* {COPYRIGHT}\n * SPDX-License-Identifier: Apache-2.0\n */\n\n'


def validate_fragment(text):
    literals = r"'(?:\\.|[^'\\])*'|\"(?:\\.|[^\"\\])*\"|/\*.*?\*/"
    remainder = re.sub(literals, '', text, flags=re.S)
    if '{' in remainder or '}' in remainder or '%code' in remainder:
        raise ValueError('Grammar fragments must not contain semantic actions or embedded code')


def grammar_text(vocabulary):
    declarations = ''.join('%token ' + name + '\n' for name in vocabulary['tokens'])
    precedence = (SOURCE / 'precedence.y').read_text()
    sections = [(SOURCE / (name + '.y')).read_text() for name in ['statements', 'expressions', 'declarations']]
    for section in [precedence, *sections]:
        validate_fragment(section)
    text = GRAMMAR_HEADER + '%start start\n' + declarations + precedence + '\n%expect 0\n%%\n' + '\n'.join(sections) + '\n%%\n'
    return normalize_grammar(text)


def normalize_grammar(text):
    """Normalize line margins of local grammar text, preserving token literals.

    Strings are call-owned; output is bounded by four times the input length.
    Only indentation tabs expand; interior bytes and newline counts stay intact.
    """
    lines = []
    for line in text.split('\n'):
        content = line.lstrip(' \t')
        indentation = line[:len(line) - len(content)].expandtabs(4)
        lines.append((indentation + content).rstrip(' \t'))
    return '\n'.join(lines)


def reduction_value(rule):
    if rule == 'accept':
        return 32767
    return -(int(rule) + 1)


def state_entries(state, symbols):
    actions = {}
    gotos = []
    for item in state.findall('./actions/transitions/transition'):
        symbol = symbols[item.get('symbol')]
        number = int(item.get('state'))
        if item.get('type') == 'shift':
            actions[symbol] = number + 1
        else:
            gotos.append((symbol, number))
    for item in state.findall('./actions/errors/error'):
        actions[symbols[item.get('symbol')]] = 0
    return actions, sorted(gotos)


def state_reductions(state, symbols, actions):
    default = 0
    for item in state.findall('./actions/reductions/reduction'):
        if item.get('enabled') == 'false':
            continue
        value = reduction_value(item.get('rule'))
        if item.get('symbol') == '$default':
            default = value
        else:
            actions[symbols[item.get('symbol')]] = value
    return default


def rust_array(name, kind, entries):
    rows = [', '.join(entries[i:i + 12]) for i in range(0, len(entries), 12)]
    return f'pub(crate) static {name}: &[{kind}] = &[\n' + ''.join('    ' + row + ',\n' for row in rows) + '];\n'


def lexical_maps(vocabulary, symbols):
    entries = [f'({json.dumps(text)}, {symbols[name]})' for text, name in sorted(vocabulary['keywords'].items())]
    output = rust_array('KEYWORDS', '(&str, u16)', entries)
    operators = sorted(vocabulary['operators'].items(), key=lambda item: (-len(item[0]), item[0]))
    entries = [f'(b{json.dumps(text)}, {symbols[name]})' for text, name in operators]
    output += rust_array('OPERATORS', '(&[u8], u16)', entries)
    entries = [f'(b{json.dumps(text)}, {symbols[name]})' for text, name in sorted(vocabulary['casts'].items())]
    return output + rust_array('CASTS', '(&[u8], u16)', entries)


def symbol_data(report, vocabulary):
    elements = report.findall('./grammar/terminals/terminal') + report.findall('./grammar/nonterminals/nonterminal')
    symbols = {item.get('name'): int(item.get('symbol-number')) for item in elements}
    declarations = vocabulary['tokens']
    tokens = [RUST_HEADER, '// Generated from independent language contracts. See LANGUAGE.md.\n']
    for name in declarations:
        tokens.append(f'pub const {name}: u16 = {symbols[name]};\n')
    punctuation = [(ord(ast.literal_eval(name)), number) for name, number in symbols.items() if name.startswith("'")]
    tokens.append(rust_array('PUNCTUATION', '(u8, u16)', list(map(str, sorted(punctuation)))))
    entries = [f'({name}, {json.dumps(name)})' for name in declarations]
    tokens.append(rust_array('TOKEN_NAMES', '(u16, &str)', entries))
    tokens.append(lexical_maps(vocabulary, symbols))
    return symbols, ''.join(tokens)


def table_data(report, symbols):
    actions, gotos, offsets, goffsets, defaults = [], [], [0], [0], []
    for state in report.findall('./automaton/state'):
        entries, jumps = state_entries(state, symbols)
        defaults.append(str(state_reductions(state, symbols, entries)))
        actions.extend(str(pair) for pair in sorted(entries.items()))
        gotos.extend(str(pair) for pair in jumps)
        offsets.append(len(actions))
        goffsets.append(len(gotos))
    output = rust_array('ACTIONS', '(u16, i16)', actions)
    output += rust_array('GOTOS', '(u16, u16)', gotos)
    output += rust_array('ACTION_OFFSETS', 'usize', list(map(str, offsets)))
    output += rust_array('GOTO_OFFSETS', 'usize', list(map(str, goffsets)))
    return output + rust_array('DEFAULTS', 'i16', defaults)


def rule_data(report, symbols):
    rules = report.findall('./grammar/rules/rule')
    entries = [str((symbols[rule.findtext('lhs')], len(rule.findall('./rhs/symbol')))) for rule in rules]
    names = {number: name for name, number in symbols.items()}
    labels = [json.dumps(names.get(number, '<invalid>')) for number in range(max(names) + 1)]
    return rust_array('RULES', '(u16, usize)', entries) + rust_array('SYMBOL_NAMES', '&str', labels)


def format_rust(text):
    result = subprocess.run(['rustfmt', '--edition', '2024', '--emit', 'stdout'], input=text, text=True, capture_output=True, check=True)
    return result.stdout


def publish(files, check):
    for path, text in files.items():
        if check:
            if path.read_text() != text:
                raise SystemExit(f'Generated file is stale: {path.relative_to(ROOT)}')
        else:
            path.write_text(text)


def generate(check=False):
    with tempfile.TemporaryDirectory(prefix='phprs-grammar-') as temporary:
        directory = Path(temporary)
        grammar = directory / 'syntax.y'
        report = directory / 'syntax.xml'
        vocabulary = json.loads(VOCABULARY.read_text())
        grammar.write_text(grammar_text(vocabulary))
        subprocess.run(['bison', '--xml=' + str(report), '-o', str(directory / 'syntax.c'), str(grammar)], check=True)
        root = ET.parse(report).getroot()
        symbols, tokens = symbol_data(root, vocabulary)
        output = RUST_HEADER + '// Generated by tools/generate_parser.py; do not edit.\n'
        output += '// Compiled from independent language contracts; see LANGUAGE.md.\n'
        output += table_data(root, symbols) + rule_data(root, symbols)
        files = {
            ROOT / 'src/syntax/tables.rs': format_rust(output),
            ROOT / 'src/syntax/token_ids.rs': format_rust(tokens),
            ROOT / 'grammar/syntax.y': grammar.read_text(),
        }
        publish(files, check)
        print(f'Generated {len(root.findall("./automaton/state"))} states and {len(root.findall("./grammar/rules/rule"))} rules.')


if __name__ == '__main__':
    if sys.argv[1:] not in ([], ['--check']):
        raise SystemExit('usage: generate_parser.py [--check]')
    generate(check=sys.argv[1:] == ['--check'])
