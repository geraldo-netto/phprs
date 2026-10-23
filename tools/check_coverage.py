#!/usr/bin/env python3
"""Enforce source-function coverage using LLVM's own annotated line counts."""
import json
import os
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def source_groups(data):
    groups = {}
    for function in data['functions']:
        filename = Path(function['filenames'][0])
        if not filename.is_relative_to(ROOT / 'src'):
            continue
        regions = [r for r in function['regions'] if r[5] == 0 and r[7] == 0]
        line, column = regions[0][:2]
        source = filename.read_text().splitlines()
        cutoff = next((i + 1 for i, text in enumerate(source) if text == '#[cfg(test)]'), len(source) + 1)
        if line >= cutoff:
            continue
        key = (filename, line, column, max(r[2] for r in regions))
        groups.setdefault(key, []).append(function['name'])
    return groups


def line_counts(text):
    counts = {}
    for match in re.finditer(r'^\s*(\d+)\|\s*([0-9.]+[kMG]?)\|', text, re.M):
        line, count = match.groups()
        counts[int(line)] = counts.get(int(line), False) or count != '0'
    return counts


def llvm_command():
    directory = Path(subprocess.check_output(['rustc', '--print', 'target-libdir'], text=True).strip())
    covered = ROOT / 'target/llvm-cov-target'
    command = [str(directory.parent / 'bin/llvm-cov'), 'show', '--format=text', '--show-line-counts', '--show-instantiations=false', '-instr-profile=' + str(covered / 'phprs.profdata')]
    objects = sorted(path for path in (covered / 'debug/deps').iterdir() if path.is_file() and not path.suffix and os.access(path, os.X_OK))
    for path in objects:
        command.extend(['-object', str(path)])
    return command


def measure_group(command, key, names):
    filename, start, _, end = key
    pattern = '^(' + '|'.join(re.escape(name) for name in names) + ')$'
    result = subprocess.run(command + ['--name-regex=' + pattern, '--sources', str(filename)], text=True, capture_output=True, check=True)
    counts = {line: hit for line, hit in line_counts(result.stdout).items() if start <= line <= end}
    if not counts:
        raise RuntimeError(f'No line counters: {filename}:{start}')
    return sum(counts.values()), len(counts)


def check_rows(rows, minimum=80):
    if not rows:
        raise RuntimeError('No production function counters were measured')
    failures = [row for row in rows if row[2] * 100 < minimum * row[3]]
    if failures:
        raise SystemExit('Functions below coverage threshold: ' + repr(failures))


def check():
    data = json.loads((ROOT / 'target/coverage.json').read_text())['data'][0]
    command = llvm_command()
    rows = []
    for key, names in sorted(source_groups(data).items()):
        covered, total = measure_group(command, key, names)
        rows.append((str(key[0].relative_to(ROOT)), key[1], covered, total))
    output = ROOT / 'target/function-coverage.tsv'
    output.write_text('file\tline\tcovered\texecutable\n' + ''.join('\t'.join(map(str, row)) + '\n' for row in rows))
    check_rows(rows)
    print(f'{len(rows)} production source functions/closures each have at least 80% executable-line coverage. Details: {output.relative_to(ROOT)}')


if __name__ == '__main__':
    check()
