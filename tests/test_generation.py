import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch
import xml.etree.ElementTree as ET
from tools import generate_parser as generator


class GrammarTests(unittest.TestCase):
    def test_action_removal_respects_c_literals(self):
        source = '''rule: '{' { if (x) { f("}"); } /* } */ f('}'); } word // {\n | %empty;'''
        self.assertEqual(generator.remove_actions(source), "rule: '{'  word \n | %empty;")

    def test_independent_grammar_preserves_precedence(self):
        source = '%code {ignore}\n%token <ast> T_X "x"\n%left T_X\n%%\nstart: T_X { f(); } | %empty;\n%%\nunused'
        result = generator.grammar_text(source)
        self.assertIn('%token  T_X "x"', result)
        self.assertIn('%left T_X', result)
        self.assertIn('start: T_X  | %empty;', result)
        self.assertNotIn('f()', result)

    def test_reduction_encoding(self):
        self.assertEqual(generator.reduction_value('accept'), 32767)
        self.assertEqual(generator.reduction_value('3'), -4)

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
        declarations = [('T_A', json.dumps("'echo'")), ('T_B', json.dumps("'++'")), ('T_C', ''), ('T_D', json.dumps("'${'"))]
        symbols = {alias: index for index, (_, alias) in enumerate(declarations)}
        result = generator.lexical_maps(declarations, symbols)
        self.assertIn('("echo", 0)', result)
        self.assertIn('(b"++", 1)', result)
        self.assertNotIn('(b"${"', result)

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
