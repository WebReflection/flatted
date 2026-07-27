#!/usr/bin/env node
/**
 * Compare flatted ESM/JS vs Rust using the same 1s wall-clock approach as test/bench.js.
 *
 *   node test/bench-js-vs-rs.mjs
 *
 * Prepares the big circular fixture, runs both benches with --json, prints a table.
 */

import { spawnSync } from 'node:child_process';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import { stringify } from '../esm/index.js';

const require = createRequire(import.meta.url);
const __dirname = dirname(fileURLToPath(import.meta.url));
const root = join(__dirname, '..');
const rustDir = join(root, 'rust');
const fixture = join(__dirname, '.bench-circular.flatted.json');
const circularTxt = join(__dirname, 'circular.txt');

function ensureFixture() {
  if (existsSync(fixture)) return;
  if (!existsSync(circularTxt)) {
    console.warn('skipping big circular fixture (test/circular.txt missing)');
    return;
  }
  process.stderr.write('Preparing flatted fixture from circular.txt...\n');
  const CircularJSON = require('circular-json');
  const data = CircularJSON.parse(readFileSync(circularTxt, 'utf8'));
  writeFileSync(fixture, stringify(data));
}

function runJson(command, args, cwd) {
  const res = spawnSync(command, args, {
    cwd,
    encoding: 'utf8',
    maxBuffer: 16 * 1024 * 1024,
  });
  if (res.status !== 0) {
    process.stderr.write(res.stderr || res.stdout || `${command} failed\n`);
    process.exit(res.status || 1);
  }
  return (res.stdout || '')
    .split('\n')
    .map((l) => l.trim())
    .filter((l) => l.startsWith('{'))
    .map((l) => JSON.parse(l));
}

ensureFixture();

process.stderr.write('Running JS (ESM) bench...\n');
const jsRows = runJson(process.execPath, [join(__dirname, 'bench-flatted.mjs'), '--json'], root);

process.stderr.write('Running Rust bench (release)...\n');
const rustRows = runJson(
  'cargo',
  ['run', '--example', 'bench', '--release', '--quiet', '--', '--json'],
  rustDir
);

const byKey = new Map();
for (const row of [...jsRows, ...rustRows]) {
  const key = `${row.case}\t${row.op}`;
  if (!byKey.has(key)) byKey.set(key, { case: row.case, op: row.op, label: row.label });
  byKey.get(key)[row.engine] = row.ops;
}

const order = [
  'objects_100',
  'objects_50',
  'objects_10',
  'shared_50',
  'shared_90',
  'circular_100',
  'circular_shared_90',
  'big_circular',
];

console.log('flatted ESM/JS vs Rust  (ops/sec, ~1s each; higher is better)');
console.log('='.repeat(78));
console.log(
  'case'.padEnd(22) +
    'op'.padEnd(12) +
    'js'.padStart(12) +
    'rust'.padStart(12) +
    'rust/js'.padStart(10)
);
console.log('-'.repeat(78));

for (const caseId of order) {
  for (const op of ['stringify', 'parse']) {
    const row = byKey.get(`${caseId}\t${op}`);
    if (!row) continue;
    const js = row.js ?? NaN;
    const rust = row.rust ?? NaN;
    const ratio = js > 0 ? rust / js : NaN;
    console.log(
      caseId.padEnd(22) +
        op.padEnd(12) +
        js.toFixed(1).padStart(12) +
        rust.toFixed(1).padStart(12) +
        (Number.isFinite(ratio) ? ratio.toFixed(2) + 'x' : 'n/a').padStart(10)
    );
  }
}

console.log('='.repeat(78));
console.log('Methodology: same as test/bench.js — count iterations in a 1s window.');
console.log('JS: esm/index.js via Node.  Rust: cargo --release example bench.');
