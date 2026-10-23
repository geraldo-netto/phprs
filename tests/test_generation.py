# Copyright 2026 Geraldo Netto
# SPDX-License-Identifier: Apache-2.0

import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch
import xml.etree.ElementTree as ET
from tools import generate_parser as generator


class GrammarTests(unittest.TestCase):
    def test_reject_embedded_actions_phprs_090(self):
        generator.validate_fragment("rule: '{' '}' ; /* { ignored } */")
        for source in ['rule: T_X { action(); };', '%code { embedded; }', 'rule: T_X } ;']:
            with self.assertRaises(ValueError):
                generator.validate_fragment(source)

    def test_independent_grammar_preserves_precedence(self):
        sections = ['%left T_X\n', 'start: T_X;\n', '', '']
        with patch.object(Path, 'read_text', side_effect=sections):
            result = generator.grammar_text({'tokens': ['T_X']})
        self.assertIn('%token T_X\n', result)
        self.assertIn('%left T_X\n', result)
        self.assertIn('start: T_X;', result)
        self.assertIn('%expect 0', result)

    def test_reduction_encoding(self):
        self.assertEqual(generator.reduction_value('accept'), 32767)
        self.assertEqual(generator.reduction_value('3'), -4)

    def test_grammar_whitespace_phprs_036(self):
        sections = ['%left T_TEXT  \n', " \tstart: T_TEXT \t\n\t| ' ' ; \t\n", 'literal: "a\tb";\n', '']
        with patch.object(Path, 'read_text', side_effect=sections):
            result = generator.grammar_text({'tokens': ['T_TEXT']})
        self.assertIn('%left T_TEXT\n', result)
        self.assertIn('    start: T_TEXT\n', result)
        self.assertIn("    | ' ' ;\n", result)
        self.assertIn('literal: "a\tb";', result)
        self.assertTrue(result.endswith('\n'))
        for line in result.splitlines():
            self.assertEqual(line, line.rstrip(' \t'))
            indentation = line[:len(line) - len(line.lstrip(' \t'))]
            self.assertNotIn('\t', indentation)

    def test_grammar_whitespace_boundaries_phprs_036(self):
        for source in ['', ' \t', '\n', '\t\n \t\n', '    x', "\t'\t' \t", 'x\ty\n']:
            result = generator.normalize_grammar(source)
            self.assertEqual(generator.normalize_grammar(result), result)
            self.assertEqual(result.count('\n'), source.count('\n'))
            self.assertLessEqual(len(result), 4 * len(source))
        self.assertEqual(generator.normalize_grammar("\t'\t' \t"), "    '\t'")

    def test_transition_and_reduction_contract(self):
        state = ET.fromstring('''<state><actions><transitions><transition symbol="token" type="shift" state="2"/><transition symbol="rule" type="goto" state="3"/></transitions><errors><error symbol="bad"/></errors><reductions><reduction symbol="token" rule="8" enabled="false"/><reduction symbol="bad" rule="4" enabled="true"/><reduction symbol="$default" rule="2" enabled="true"/></reductions></actions></state>''')
        symbols = {'token': 1, 'rule': 2, 'bad': 3}
        actions, gotos = generator.state_entries(state, symbols)
        self.assertEqual(actions, {1: 3, 3: 0})
        self.assertEqual(gotos, [(2, 3)])
        self.assertEqual(generator.state_reductions(state, symbols, actions), -3)
        self.assertEqual(actions, {1: 3, 3: -5})

    def test_array_and_lexer_maps(self):
        self.assertIn('static EMPTY', generator.rust_array('EMPTY', 'u8', []))
        self.assertIn('12,', generator.rust_array('LONG', 'u8', list(map(str, range(13)))))
        vocabulary = {'keywords': {'echo': 'T_A'}, 'operators': {'++': 'T_B'}, 'casts': {'int': 'T_C'}}
        result = generator.lexical_maps(vocabulary, {'T_A': 0, 'T_B': 1, 'T_C': 2})
        self.assertIn('("echo", 0)', result)
        self.assertIn('(b"++", 1)', result)
        self.assertIn('(b"int", 2)', result)
        self.assertIn('static CASTS', result)

    def test_sparse_rule_labels_phprs_014(self):
        report = ET.fromstring('<report><grammar><rules><rule><lhs>start</lhs><rhs><symbol>x</symbol></rhs></rule></rules></grammar></report>')
        result = generator.rule_data(report, {'x': 0, 'start': 2})
        self.assertIn('"x", "<invalid>", "start"', result)

    def test_rust_formatting(self):
        self.assertIn('const X: u8 = 1;', generator.format_rust('const X:u8=1;'))
        with self.assertRaises(subprocess.CalledProcessError):
            generator.format_rust('const = ;')

    def test_publish_rejects_stale_artifact(self):
        with tempfile.TemporaryDirectory(dir=generator.ROOT / 'target') as directory:
            path = Path(directory) / 'generated.rs'
            files = {path: 'expected'}
            generator.publish(files, False)
            generator.publish(files, True)
            path.write_text('corrupt')
            with self.assertRaises(SystemExit):
                generator.publish(files, True)

    def test_grammar_reproducibility_phprs_013(self):
        generator.generate(check=True)


if __name__ == '__main__':
    unittest.main()
