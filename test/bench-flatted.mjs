// Flatted-only bench mirroring test/bench.js timing (1s wall-clock ops/sec).
// Used by test/bench-js-vs-rs.mjs for JS vs Rust comparison.
//
//   node test/bench-flatted.mjs
//   node test/bench-flatted.mjs --json

import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import { parse, stringify } from '../esm/index.js';

const require = createRequire(import.meta.url);
const __dirname = dirname(fileURLToPath(import.meta.url));
const jsonMode = process.argv.includes('--json');

const WINDOW_MS = 1000;

function bench(method, dummy) {
  const t = Date.now();
  let i = 0;
  let r;
  while (Date.now() - t < WINDOW_MS) {
    r = method(dummy);
    i++;
  }
  return { ops: i / (WINDOW_MS / 1000), last: r };
}

function emit(caseId, op, label, ops) {
  if (jsonMode) {
    console.log(JSON.stringify({ engine: 'js', case: caseId, op, label, ops: +ops.toFixed(2) }));
  } else {
    console.log(`js / flatted ${op} ${label} parsed ${ops.toFixed(2)} times per second`);
  }
}

function pair(caseId, label, value) {
  const s = bench(stringify, value);
  emit(caseId, 'stringify', label, s.ops);
  const p = bench(parse, s.last);
  emit(caseId, 'parse', label, p.ops);
  return s.last;
}

const dummyAll = JSON.parse(readFileSync(join(__dirname, 'data.json'), 'utf8'));
const keys = Object.keys(dummyAll[0]).length;

if (!jsonMode) {
  console.log('-----------------------------------');
  console.log('Object with ' + keys + ' keys each');
  console.log('-----------------------------------');
}

let dummy100 = dummyAll;
let dummy50 = dummyAll.slice(0, 50);
let dummy10 = dummyAll.slice(0, 10);

pair('objects_100', '100 objects', dummy100);
pair('objects_50', '50 objects', dummy50);
pair('objects_10', '10 objects', dummy10);

if (!jsonMode) {
  console.log('-----------------------------------');
  console.log('50% same objects');
  console.log('-----------------------------------');
}
dummy100 = dummy50.concat(dummy50);
pair('shared_50', '100 objects', dummy100);

if (!jsonMode) {
  console.log('-----------------------------------');
  console.log('90% same objects');
  console.log('-----------------------------------');
}
dummy100 = [].concat(
  dummy10, dummy10, dummy10, dummy10, dummy10,
  dummy10, dummy10, dummy10, dummy10, dummy10
);
pair('shared_90', '100 objects', dummy100);

if (!jsonMode) {
  console.log('-----------------------------------');
  console.log('with circular');
  console.log('-----------------------------------');
}
function makeCircularObject() {
  const a = {};
  a.b = a;
  return a;
}
dummy100 = [];
for (let i = 0; i < 100; i++) dummy100.push(makeCircularObject());
pair('circular_100', '100 objects', dummy100);

if (!jsonMode) {
  console.log('-----------------------------------');
  console.log('with circular 90% same');
  console.log('-----------------------------------');
}
dummy10 = [];
for (let i = 0; i < 10; i++) dummy10.push(makeCircularObject());
dummy100 = [].concat(
  dummy10, dummy10, dummy10, dummy10, dummy10,
  dummy10, dummy10, dummy10, dummy10, dummy10
);
pair('circular_shared_90', '100 objects', dummy100);

const circularTxt = join(__dirname, 'circular.txt');
const flattedFixture = join(__dirname, '.bench-circular.flatted.json');

if (existsSync(circularTxt)) {
  if (!jsonMode) {
    console.log('-----------------------------------');
    console.log('Big real-world circular data');
    console.log('-----------------------------------');
  }
  // circular.txt is CircularJSON; convert once to flatted for fair JS/Rust parity.
  let flattedText;
  if (existsSync(flattedFixture)) {
    flattedText = readFileSync(flattedFixture, 'utf8');
  } else {
    const CircularJSON = require('circular-json');
    const cirular = CircularJSON.parse(readFileSync(circularTxt, 'utf8'));
    flattedText = stringify(cirular);
    writeFileSync(flattedFixture, flattedText);
  }
  const value = parse(flattedText);
  const label = flattedText.length + ' chars';
  emit('big_circular', 'stringify', label, bench(stringify, value).ops);
  emit('big_circular', 'parse', label, bench(parse, flattedText).ops);
}
