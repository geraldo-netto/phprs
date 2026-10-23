# Copyright 2026 Geraldo Netto
# SPDX-License-Identifier: Apache-2.0

"""Language-level vocabulary checked without PHP implementation sources."""
import json
from pathlib import Path
import subprocess
import unittest

ROOT = Path(__file__).resolve().parents[1]


class VocabularyTests(unittest.TestCase):
    def setUp(self):
        self.vocabulary = json.loads((ROOT / 'grammar/vocabulary.json').read_text())

    def test_independent_token_names(self):
        names = self.vocabulary['tokens']
        self.assertEqual(names, sorted(set(names)))
        for name in names:
            self.assertRegex(name, r'^T_[A-Z_]+$')
        self.assertFalse(any(isinstance(name, int) for name in names))

    def test_spellings_reference_declared_classes(self):
        names = set(self.vocabulary['tokens'])
        for section in ['keywords', 'operators', 'casts', 'compound_keywords']:
            entries = self.vocabulary[section]
            self.assertEqual(len(entries), len({text.lower() for text in entries}))
            for text, name in entries.items():
                self.assertTrue(text.isascii() and 0 < len(text) <= 32)
                self.assertIn(name, names)

    def test_shared_public_tokenizer_observations(self):
        newer = set(self.vocabulary['provenance']['documented_php85_tokens'])
        cases = []
        for text, name in self.vocabulary['keywords'].items():
            if name not in newer:
                suffix = ' X' if text == 'enum' else ''
                cases.extend([(text + suffix, name), (text.upper() + suffix, name)])
        for text, name in self.vocabulary['operators'].items():
            if name not in newer:
                cases.append((text, name))
        for text, name in self.vocabulary['casts'].items():
            if name not in newer:
                cases.append(('(' + text + ')', name))
        self.compare_observations(cases)

    def compare_observations(self, cases):
        program = '$out=[];foreach(json_decode(stream_get_contents(STDIN)) as $text){$ts=token_get_all("<?php ".$text);$t=$ts[1];$out[]=is_array($t)?token_name($t[0]):$t;}echo json_encode($out);'
        result = subprocess.run(['php', '-r', program], input=json.dumps([text for text, _ in cases]), text=True, capture_output=True, check=True)
        actual = json.loads(result.stdout)
        self.assertEqual(actual, [name for _, name in cases])
