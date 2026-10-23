from pathlib import Path
import unittest
from unittest.mock import patch
from tools import check_coverage as checker


class CoverageTests(unittest.TestCase):
    def test_llvm_line_counts(self):
        text = ' 1| 0|uncovered\n 2| 1.2k|covered\n 3| |comment\n 1| 2|covered instance\n'
        self.assertEqual(checker.line_counts(text), {1: True, 2: True})

    def test_threshold_is_individual(self):
        checker.check_rows([('good', 1, 8, 10)])
        with self.assertRaises(SystemExit):
            checker.check_rows([('large', 1, 10000, 10000), ('small', 1, 0, 1)])

    def test_no_counters_cannot_pass_phprs_031(self):
        with self.assertRaises(RuntimeError):
            checker.check_rows([])

    def test_source_function_grouping(self):
        source = 'fn example() { 1; }\n#[cfg(test)]\nfn test() {}\n'
        function = {'filenames': [str(checker.ROOT / 'src/example.rs')], 'name': 'instance1', 'regions': [[1, 1, 1, 20, 1, 0, 0, 0]]}
        second = dict(function, name='instance2')
        test = dict(function, name='test', regions=[[3, 1, 3, 10, 1, 0, 0, 0]])
        outside = dict(function, filenames=[str(checker.ROOT / 'tests/example.rs')])
        with patch.object(Path, 'read_text', return_value=source):
            groups = checker.source_groups({'functions': [function, second, test, outside]})
        self.assertEqual(list(groups.values()), [['instance1', 'instance2']])

    def test_measure_uses_native_counts(self):
        key = (Path('/source.rs'), 1, 1, 3)
        with patch.object(checker.subprocess, 'run') as run:
            run.return_value.stdout = ' 1| 1|covered\n 2| 0|missed\n 4| 1|outside\n'
            self.assertEqual(checker.measure_group(['llvm-cov'], key, ['name']), (1, 2))
            run.return_value.stdout = ''
            with self.assertRaises(RuntimeError):
                checker.measure_group(['llvm-cov'], key, ['name'])

    def test_actual_source_functions(self):
        checker.check()


if __name__ == '__main__':
    unittest.main()
